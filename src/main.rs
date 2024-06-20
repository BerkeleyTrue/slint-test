slint::include_modules!();

fn main() {
    use slint::Model;

    let app_window = AppWindow::new().unwrap();
    let mut tiles: Vec<TileData> = app_window.get_memory_tiles().iter().collect();
    tiles.extend(tiles.clone());

    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    app_window.set_memory_tiles(tiles_model.clone().into());

    let app_window_weak = app_window.as_weak();
    app_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            if t1 == t2 {
                t1.solved = true;
                t2.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let app_window = app_window_weak.unwrap();
                app_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    app_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    t2.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    tiles_model.set_row_data(t2_idx, t2);
                })
            }
        }
    });

    app_window.run().unwrap()
}
