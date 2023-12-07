// If the file is nested inside another directory
mod bin {
    pub mod exercise_01;
}
// Otherwise you can simple use the following
// mod exercise_01;

fn main() {
    first_exercise();
}

fn first_exercise() {
    let first_name: &str = "Nicolas";
    let last_name: &str = "Ribeiro";

    /*
     * Either import like this
     * bin::exercise_01::display_names(first_name, last_name);
     *
     * Or with the *USE* keyword
     * use bin::exercise_01::display_names
     *
     * and call the function directly
     * display_names(first_name, last_name);
     */
    // Obs, you the file is nested in another directory you have to prefix with the directory name and ::
    exercise_01::display_names(first_name, last_name);
}
