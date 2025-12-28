//! Reusable UI components - Buttons, bars, cards, etc.

use leptos::prelude::*;

/// A stat bar component showing a label, current/max values, and a colored progress bar
#[component]
pub fn StatBar(
    /// The label for the stat (e.g., "HP", "Chakra")
    label: &'static str,
    /// Current value of the stat
    current: i32,
    /// Maximum value of the stat
    max: i32,
    /// Color of the bar (CSS color string)
    color: &'static str,
) -> impl IntoView {
    let percentage = if max > 0 {
        (current as f64 / max as f64) * 100.0
    } else {
        0.0
    };

    view! {
        <div class="stat-bar">
            <div class="stat-label">
                <span>{label}</span>
                <span class="stat-values">{current}"/" {max}</span>
            </div>
            <div class="stat-bar-bg">
                <div
                    class="stat-bar-fill"
                    style:width=format!("{}%", percentage)
                    style:background-color=color
                />
            </div>
        </div>
    }
}
