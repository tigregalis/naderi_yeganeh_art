use naderi_yeganeh_art::*;

fn main() {
    run::run::<{ sunflower_field::FULL_M }, { sunflower_field::FULL_N }>(sunflower_field::draw)
}
