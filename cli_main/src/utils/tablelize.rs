type FnColor = dyn Fn(String) -> Result<(), std::io::Error>;

// colorize: frame, label, numeric, over
// table: label, ddl, over
pub fn item_list(
    table: Vec<(String, String, bool)>,
    colorize: (&FnColor, &FnColor, &FnColor, &FnColor),
    node: (u16, u16),
) -> Result<(), std::io::Error> {
    let (frame, label, numeric, over) = colorize;

    let frame_literal = "+----+------------------+-------+-------\n";

    let mut top_frame = String::from(frame_literal);
    let top_frame_b = unsafe { top_frame.as_bytes_mut() };
    if node.0 < 40 {
        top_frame_b[node.0 as usize] = b'+';
    }
    if node.1 < 40 {
        top_frame_b[node.1 as usize] = b'+';
    }
    frame(top_frame)?;

    frame("| ID |        DDL       | Over? | Label \n".to_string())?;
    frame(frame_literal.to_string())?;

    for (i, item) in table.iter().enumerate() {
        frame("|".to_string())?;
        let item_color = if item.2 { &over } else { &numeric };
        if i >= 999 {
            item_color(format!("{}", i + 1))?;
        } else if i >= 99 {
            item_color(format!("{} ", i + 1))?;
        } else {
            item_color(format!(" {:02} ", i + 1))?;
        }

        frame("| ".to_string())?;
        item_color(item.1.clone())?;

        frame(" |".to_string())?;
        item_color(if item.2 {
            " Over! ".to_string()
        } else {
            "       ".to_string()
        })?;

        frame("| ".to_string())?;
        (if item.2 { over } else { label })(item.0.clone())?;

        print!("\n");
    }
    frame(frame_literal.to_string())?;
    Ok(())
}

// colorize: frame, content
pub fn entry_head(
    id: usize,
    entry: String,
    colorize: (&FnColor, &FnColor),
) -> Result<(u16, u16), std::io::Error> {
    let (frame, body) = colorize;

    frame("+------+-".to_string())?;
    for _ in 0..entry.len() {
        frame("-".to_string())?;
    }
    frame("-+\n|".to_string())?;
    body(format!(" {:04} ", id))?;
    frame("|".to_string())?;
    body(format!(" {} ", entry))?;
    frame("|\n".to_string())?;

    Ok((7, 10 + entry.len() as u16))
}
