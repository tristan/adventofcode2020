use adventofcode2020::read_input_lines;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input_lines("day_10_input.txt")?;
    let result = part1(input.clone());
    println!("part1: {}", result);
    let result = part2(input);
    println!("part2: {}", result);
    Ok(())
}

fn part1(mut input: Vec<usize>) -> usize {
    input.push(0);
    input.sort();

    let (one, three) = input.windows(2).fold((0, 0), |(one, three), pair| {
        let diff = pair[1] - pair[0];
        if diff == 1 {
            (one + 1, three)
        } else if diff == 3 {
            (one, three + 1)
        } else {
            (one, three)
        }
    });

    one * (three + 1)
}

fn part2(mut input: Vec<usize>) -> usize {
    input.push(0);
    input.sort();
    input.push(input[input.len() - 1] + 3);

    let mut weights = vec![0; input.len()];
    weights[0] = 1;
    for i in 0..(input.len() - 1) {
        for j in 1.. {
            if i + j < input.len() && input[i + j] - input[i] <= 3 {
                weights[i + j] += weights[i];
            } else {
                break;
            }
        }
    }
    weights[weights.len() - 1]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() -> Result<(), Box<dyn std::error::Error>> {

        let input = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        ];

        assert_eq!(part1(input), 220);

        Ok(())
    }

    #[test]
    fn test_2() -> Result<(), Box<dyn std::error::Error>> {
        let input = vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        ];

        let result = part2(input);

        assert_eq!(8, result);

        let input = vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        ];

        let result = part2(input);

        assert_eq!(19208, result);


        Ok(())
    }
}
