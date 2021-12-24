use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut burrow: Vec<Vec<char>> = input
        .lines()
        .skip(1)
        .map(|l| l.chars().skip(1).collect())
        .collect();

    let result = shortest_path(&burrow);
    writeln!(io::stdout(), "Part1: {}", result)?;

    burrow.insert(2, " #D#C#B#A#".chars().collect());
    burrow.insert(3, " #D#B#A#C#".chars().collect());


    let result = shortest_path(&burrow);
    writeln!(io::stdout(), "Part2: {}", result)?;

    Ok(())
}

fn moves(burrow: &Vec<Vec<char>>) -> Vec<(usize, Vec<Vec<char>>)> {
    // 计算所有片脚类的所有可能移动情况
    let mut burrows = vec![];
    burrows.extend(move_out(burrow));
    burrows.extend(move_in(burrow));
    burrows
}

fn swap_pos(burrow: &Vec<Vec<char>>, (i, j): (usize, usize), (x, y): (usize, usize)) -> Vec<Vec<char>> {
    let mut new_burrow = burrow.clone();
    new_burrow[i][j] = burrow[x][y];
    new_burrow[x][y] = burrow[i][j];
    new_burrow
}

fn move_in(burrow: &Vec<Vec<char>>) -> Vec<(usize, Vec<Vec<char>>)> {
    // 计算当前情况中处于过道中的所有片脚类能够移动到房间中的所有情况
    let mut burrows = vec![];
    let room_bound = burrow.len() - 1;

    for j in 0..11 {
        // 针对所有处在过道中的片脚类
        let (dest_room, energy) = match burrow[0][j] {
            'A' => (2, 1),
            'B' => (4, 10),
            'C' => (6, 100),
            'D' => (8, 1000),
            _ => continue,
        };

        // 计算当前片脚类能进入目标房间的最远位置。
        let dest_i = match (1..room_bound).take_while(|&i| burrow[i][dest_room] == '.').last() {
            Some(i) => i,
            _ => continue,
        };

        // 根据规则2，判断目标房间是否还有其他的片脚类，假如存在那么将无法进行移动，跳过。
        if !(dest_i + 1..room_bound).all(|i| burrow[i][dest_room] == burrow[0][j]) {
            continue;
        }
        
        // 计算片脚类进入目标房间是需要向左移动还是向右移动。
        let (min, max) = if j < dest_room { (j, dest_room) } else { (dest_room, j) };
        // 假如在移动的过程中存在任何的其他片脚类，根据规则2，将无法进行移动，跳过
        if (min + 1..max).any(|y| burrow[0][y] != '.') {
            continue;
        }
        // 计算移动的距离和消耗的能量
        let total_energy = (max - min + dest_i) * energy;
        // 将该片脚类的移动和花费的能量存入所有可能的下一步情况中
        burrows.push((total_energy, swap_pos(burrow, (0, j), (dest_i, dest_room))));
    }

    burrows
}

fn move_out(burrow: &Vec<Vec<char>>) -> Vec<(usize, Vec<Vec<char>>)> {
    // 计算当前情况中处于房间中的所有片脚类能够移动到过道中的所有情况
    let mut burrows = vec![];
    let room_bound = burrow.len() - 1;

    for i in 1..room_bound {    // 范围限定在房间内
        for j in [2, 4, 6, 8] {
            // 假如当前位置的上方，存在其他的片脚类，那么该片脚类无法移动
            // 选择房间内的第一个片脚类
            if (1..i).any(|x| burrow[x][j] != '.') {
                continue;
            }

            // 假如当前房间内，从当前位置到房间尾部存在开放空间，那么该房间内必然不存在其他片脚类。
            if (i..room_bound).any(|x| burrow[x][j] == '.') {
                continue;
            }

            // 只针对房间内的片脚类
            let (dest_room, energy) = match burrow[i][j] {
                'A' => (2, 1),
                'B' => (4, 10),
                'C' => (6, 100),
                'D' => (8, 1000),
                _ => unreachable!("There should be a amphipods: {:?}", (i, j)),
            };
            
            // 假如当前的片脚类已经处在目标房间中
            // 且从当前位置到房间的剩余位置都是当前片脚类时
            // 不移动这个片脚类
            if dest_room == j && (i..room_bound).all(|x| burrow[x][dest_room] == burrow[i][j]) {
                continue;
            }

            // 考虑片脚类走出房间在过道上的情况
            
            // 向左移动，以从近到远的顺序考虑所有可能的位置
            for dest_j in (0..j).rev() {
                // 根据规则1，片脚类不允许停留在房间的门口
                if [2, 4, 6, 8].contains(&dest_j) {
                    continue;
                }

                // 假如过道的目标位置是非开放状态，那么该片脚类将无法移动到该位置
                // 因为是从近到远，那么更远的位置也无需考虑了，到达更远的位置时，肯定会经过较近的位置
                // 所以直接 break 即可
                if burrow[0][dest_j] != '.' {
                    break;
                }
                let total_energy = (i + j - dest_j) * energy;
                burrows.push((total_energy, swap_pos(burrow, (i, j), (0, dest_j))));
            }


            // 向右移动，与向左移动类似
            for dest_j in j..11 {
                if [2, 4, 6, 8].contains(&dest_j) {
                    continue;
                }
                if burrow[0][dest_j] != '.' {
                    break;
                }
                let total_energy = (i + dest_j - j) * energy;
                burrows.push((total_energy, swap_pos(burrow, (i, j), (0, dest_j))));
            }
        }
    }

    burrows
}

fn is_orginized(burrow: &Vec<Vec<char>>) -> bool {
    // 判断当前状态是否为目标状态
    burrow
        .iter()
        .filter(|row| row[2..9].iter().collect::<String>() == "A#B#C#D")
        .count()
        == burrow.len() - 2
}

fn shortest_path(burrow: &Vec<Vec<char>>) -> usize {
    // 利用HashMap来记录已经遍历过的情况和对应的能量值
    let mut visited = HashMap::new();

    // 利用最小堆来存储所有可能的下一次情况
    let mut min_queue = BinaryHeap::new();
    min_queue.push(Reverse((0, burrow.clone())));
    while let Some(Reverse((energy, burrow))) = min_queue.pop() {
        // 下一次情况中消耗能量最小的情况，假如满足目标状态，那么就是所求的最优解
        if is_orginized(&burrow) {
            return energy;
        }

        // 判断当前情况是否出现过，假如出现过而且当前情况的能量消耗大于出现过的情况的能量消耗
        // 跳过
        if let Some(&visited_energy) = visited.get(&burrow) {
            if visited_energy < energy {
                continue;
            }
        }


        // 遍历所有可能的下一步情况
        for (next_energy, next_burrow) in moves(&burrow) {
            let total_energy = energy + next_energy;
            let &visited_energy = visited.get(&next_burrow).unwrap_or(&1000000);
            // 判断下一步情况是否出现过
            // 假如出现过而且当前情况的能量消耗大于出现过的情况的能量消耗，跳过
            // 否则将下一步的情况和消耗的能量存入 visited 和最小堆中。
            if visited_energy > total_energy {
                visited.insert(next_burrow.clone(), total_energy);
                min_queue.push(Reverse((total_energy, next_burrow)));
            }
        }
    }
    unreachable!("There is no possible way, please check")
}
