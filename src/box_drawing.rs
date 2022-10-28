pub enum BoxDrawing {
    Light,
    Heavy,
    DoubleDashLight,
    DoubleDashHeavy,
    TripleDashLight,
    TripleDashHeavy,
    QuadrupleDashLight,
    QuadrupleDashHeavy,
    Double,
}

impl BoxDrawing {
    pub fn get_char(typ: BoxDrawing) -> (char, char, char, char, char, char) {
        match typ {
            BoxDrawing::Light =>                {('─', '│', '└', '┌', '┘', '┐')},
            BoxDrawing::Heavy =>                {('━', '┃', '┗', '┏', '┛', '┓')},
            BoxDrawing::DoubleDashLight =>      {('╌', '╎', '└', '┌', '┘', '┐')},
            BoxDrawing::DoubleDashHeavy =>      {('╍', '╏', '┗', '┏', '┛', '┓')},
            BoxDrawing::TripleDashLight =>      {('┄', '┆', '└', '┌', '┘', '┐')},
            BoxDrawing::TripleDashHeavy =>      {('┅', '┇', '┗', '┏', '┛', '┓')},
            BoxDrawing::QuadrupleDashLight =>   {('┈', '┊', '└', '┌', '┘', '┐')},
            BoxDrawing::QuadrupleDashHeavy =>   {('┉', '┋', '┗', '┏', '┛', '┓')},
            BoxDrawing::Double =>               {('═', '║', '╚', '╔', '╝', '╗')},
        }
    }
}