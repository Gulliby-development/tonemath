pub fn show(text: &str) -> String {
    let mut shown = String::new();
    shown.push('"');
    for character in text.chars() {
        if ('!'..='~').contains(&character) {
            shown.push(character);
        } else {
            shown.push('?');
        }
    }
    shown.push('"');
    shown
}

pub fn or_list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [item] => item.clone(),
        [first, second] => format!("{first} or {second}"),
        _ => {
            let (last, earlier) = items.split_last().expect("non-empty list");
            format!("{} or {last}", earlier.join(", "))
        }
    }
}

pub fn plural(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

pub fn render_table(header: &[&str], rows: &[Vec<String>]) -> Vec<String> {
    let columns = rows.iter().map(Vec::len).fold(header.len(), usize::max);
    let mut widths = vec![0; columns];

    for (column, cell) in header.iter().enumerate() {
        widths[column] = widths[column].max(cell.chars().count());
    }
    for row in rows {
        for (column, cell) in row.iter().enumerate() {
            widths[column] = widths[column].max(cell.chars().count());
        }
    }

    let mut rendered = Vec::with_capacity(rows.len() + 1);
    rendered.push(render_row(header.iter().copied(), columns, &widths));
    for row in rows {
        rendered.push(render_row(row.iter().map(String::as_str), columns, &widths));
    }
    rendered
}

fn render_row<'a>(
    cells: impl Iterator<Item = &'a str>,
    columns: usize,
    widths: &[usize],
) -> String {
    let cells: Vec<&str> = cells.collect();
    let mut line = String::new();
    for (column, width) in widths.iter().enumerate() {
        if column > 0 {
            line.push_str("  ");
        }
        let cell = cells.get(column).copied().unwrap_or("");
        line.push_str(cell);
        if column + 1 < columns {
            for _ in cell.chars().count()..*width {
                line.push(' ');
            }
        }
    }
    while line.ends_with(' ') {
        line.pop();
    }
    line
}
