pub fn answer_1(steps: usize) -> u32 {
    let mut last_pos = 0;
    let mut buf: Vec<u32> = Vec::with_capacity(2018);
    buf.push(0);

    for x in 1..2018 {
        let pos = wrap_position(buf.len(), last_pos + steps) + 1;
        buf.insert(pos, x as u32);
        last_pos = pos;
    }

    buf[last_pos + 1]
}

pub fn answer_2(steps: usize) -> u32 {
    let mut last_pos = 0;
    let mut value = 0;

    for x in 1..50000000 {
        let pos = wrap_position(x as usize, last_pos + steps) + 1;
        last_pos = pos;
        if pos == 1 {
            value = x
        }
    }

    value
}

fn wrap_position(len: usize, pos: usize) -> usize {
    match len {
        0 => 0,
        1 => 0,
        _ => pos % len,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(638, answer_1(3));
    }
}
