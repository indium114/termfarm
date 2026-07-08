static BASE_PLOT_PRICE: f64 = 100.0;
static PLOT_PRICE_GROWTH: f64 = 1.5;

pub fn next_plot_price(plot_count: u16) -> u16 {
    (BASE_PLOT_PRICE * PLOT_PRICE_GROWTH.powf((plot_count - 3) as f64)) as u16
}
