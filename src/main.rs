use js_sys::Date as JsDate;
use yew::prelude::*;

// ---------- Types ----------
#[derive(Clone, Copy, PartialEq)]
struct Ymd {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Clone, Copy, PartialEq)]
struct YmdHms {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millis: u32,
}

#[derive(Default, Clone, Copy)]
struct Diff {
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
}

// ---------- App ----------
#[function_component(App)]
fn app() -> Html {
    let now = use_state_eq(local_now);

    // tick every second
    {
        let now = now.clone();
        use_effect_with((), move |_| {
            use gloo_timers::callback::Interval;
            let handle = Interval::new(1000, move || now.set(local_now()));
            || drop(handle)
        });
    }

    let (tg_date, xmas_date) = (
        thanksgiving_next_or_same((*now).year),
        christmas_next_or_same((*now).year, (*now).month, (*now).day),
    );

    let tg_diff = diff_components(*now, tg_date);
    let xmas_diff = diff_components(*now, xmas_date);

    html! {
        <>
            <header>
                <span class="dot"/> 
                <h1>{"Dual Holiday Countdown — Thanksgiving 🦃 & Christmas 🎄"}</h1>
            </header>
            <p class="subtitle">{"Auto-updating every year, in your local time. Powered by Rust + Yew (WASM)."}</p>

            <div class="grid">
                <div class="card">
                    <div class="label"><span class="pill" style="background:var(--accent)"></span>{"Next Thanksgiving (US)"}</div>
                    <div class="t">{format!("{:02}d {:02}h {:02}m {:02}s", tg_diff.days, tg_diff.hours, tg_diff.minutes, tg_diff.seconds)}</div>
                    <div class="units">{format!("{}/{}/{} — 4th Thursday in November", tg_date.year, tg_date.month, tg_date.day)}</div>
                    <div class="quip">{thanksgiving_quip(tg_diff.days)}</div>
                </div>

                <div class="card">
                    <div class="label"><span class="pill" style="background:var(--accent2)"></span>{"Christmas"}</div>
                    <div class="t">{format!("{:02}d {:02}h {:02}m {:02}s", xmas_diff.days, xmas_diff.hours, xmas_diff.minutes, xmas_diff.seconds)}</div>
                    <div class="units">{format!("{}/{}/{} — fixed on Dec 25", xmas_date.year, xmas_date.month, xmas_date.day)}</div>
                    <div class="quip">{christmas_quip(xmas_diff.days)}</div>
                </div>
            </div>

            <footer>{"Tip: Deploy as static files. No servers, no cron, just cheer."}</footer>
        </>
    }
}

// ---------- Time helpers ----------
fn local_now() -> YmdHms {
    let d = JsDate::new_0();
    YmdHms {
        year: d.get_full_year() as i32,
        month: (d.get_month() + 1) as u32, // JS months 0–11
        day: d.get_date() as u32,
        hour: d.get_hours() as u32,
        minute: d.get_minutes() as u32,
        second: d.get_seconds() as u32,
        millis: d.get_milliseconds() as u32,
    }
}

fn date_to_ms(y: i32, m: u32, d: u32, hh: u32, mm: u32, ss: u32, ms: u32) -> i64 {
    let js = JsDate::new_with_year_month_day_hr_min_sec_milli(
        y as u32,             // <- u32 here
        (m as i32) - 1,       // month: i32 (0-based)
        d as i32,
        hh as i32,
        mm as i32,
        ss as i32,
        ms as i32,
    );
    js.get_time() as i64
}

fn weekday_of(year: i32, month: u32, day: u32) -> i32 {
    // 0=Sun .. 6=Sat
    let js = JsDate::new_with_year_month_day(year as u32, (month as i32) - 1, day as i32);
    js.get_day() as i32
}

fn compare_ymd(a: YmdHms, b: Ymd) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    match a.year.cmp(&b.year) {
        Less => return Less,
        Greater => return Greater,
        _ => {}
    }
    match a.month.cmp(&b.month) {
        Less => return Less,
        Greater => return Greater,
        _ => {}
    }
    match a.day.cmp(&b.day) {
        Less => return Less,
        Greater => return Greater,
        _ => {}
    }
    Equal
}

// ---------- Holiday logic ----------
fn fourth_thursday_of_november(year: i32) -> Ymd {
    // weekday of Nov 1, then jump to 4th Thursday
    let first_weekday = weekday_of(year, 11, 1); // 0=Sun .. 6=Sat
    let thursday = 4; // 0=Sun
    let mut day = 1 + ((7 + thursday - first_weekday) % 7) + 7 * 3; // 1st Thu + 3 weeks
    if day > 30 {
        day -= 7;
    }
    Ymd {
        year,
        month: 11,
        day: day as u32,
    }
}

fn thanksgiving_next_or_same(year_now: i32) -> Ymd {
    let mut y = year_now;
    let this = fourth_thursday_of_november(y);
    let today = local_now();
    if compare_ymd(today, this) == std::cmp::Ordering::Greater {
        y += 1;
    }
    fourth_thursday_of_november(y)
}

fn christmas_next_or_same(year_now: i32, m_now: u32, d_now: u32) -> Ymd {
    let mut y = year_now;
    if m_now > 12 || (m_now == 12 && d_now > 25) {
        y += 1;
    }
    Ymd {
        year: y,
        month: 12,
        day: 25,
    }
}

// ---------- Diff ----------
fn diff_components(now: YmdHms, target: Ymd) -> Diff {
    let now_ms = date_to_ms(
        now.year, now.month, now.day, now.hour, now.minute, now.second, now.millis,
    );
    let tgt_ms = date_to_ms(target.year, target.month, target.day, 0, 0, 0, 0);
    let mut ms = (tgt_ms as i128) - (now_ms as i128);
    if ms < 0 {
        ms = 0;
    }

    let sec = ms / 1000;
    let days = sec / 86_400;
    let hours = (sec % 86_400) / 3600;
    let minutes = (sec % 3600) / 60;
    let seconds = sec % 60;

    Diff {
        days: days as i64,
        hours: hours as i64,
        minutes: minutes as i64,
        seconds: seconds as i64,
    }
}

// ---------- Flavor text ----------
fn thanksgiving_quip(days_left: i64) -> String {
    match days_left {
        0 => "🦃 It’s today! Pace yourself—gravy is not a beverage.".into(),
        1 => "🦃 Tomorrow: begin strategic pie negotiations.".into(),
        2..=6 => "🦃 Prep mode: assign a stuffing CTO and a cranberry PM.".into(),
        7..=30 => "🦃 Consider a mashed-potato proof of concept.".into(),
        _ => "🦃 Training season: practice polite seconds-refusal face.".into(),
    }
}

fn christmas_quip(days_left: i64) -> String {
    match days_left {
        0 => "🎄 It’s today! Batteries not included—coffee mandatory.".into(),
        1 => "🎄 Tomorrow: act surprised like a pro.".into(),
        2..=6 => "🎄 Stealth wrap operations entering code freeze.".into(),
        7..=30 => "🎄 Calibrating carols. Jingle bells in QA.".into(),
        _ => "🎄 Elf standup at 9. Santa ships on 12/25.".into(),
    }
}

// ---------- Entry ----------
fn main() {
    yew::Renderer::<App>::new().render();
}