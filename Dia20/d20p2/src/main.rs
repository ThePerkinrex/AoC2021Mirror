use std::collections::HashSet;

type Image = HashSet<(isize, isize)>;

fn get_or_default(image: &Image, x: isize, y: isize, on: bool) -> bool {
    image.contains(&(x, y)) == on
}

fn get_close(image: &Image, x: isize, y: isize, on: bool) -> usize {
    let mut res: usize = 0;
    let mut i: usize = 9;
    for y in (-1 + y)..=(1 + y) {
        for x in (-1 + x)..=(1 + x) {
            // println!("{} {}", x, y);
            i -= 1;
            if get_or_default(image, x, y, on) {
                // if debug{print!("#");}
                res += 1 << i;
            }
            // else{
            //     if debug {print!(".");}
            // }
        }
        // if debug {println!()}
    }
    // if debug {
    //     println!();
    // }
    // let get  = |r, c| if get_or_default(image, c, r) {1usize} else {0usize};
    // let ((r,c)) = (y,x);
    // let n = (get(r-1, c-1)<<8) + (get(r-1, c)<<7) + (get(r-1, c+1)<<6) +
    //                     (get(r,   c-1)<<5) + (get(r,   c)<<4) + (get(r,   c+1)<<3) +
    //                     (get(r+1, c-1)<<2) + (get(r+1, c)<<1) + (get(r+1, c+1));
    // assert_eq!(res, n);
    // println!("#{}", res);
    res
}

fn apply(index: &[bool], image: Image, on: bool) -> Image {
    let mut new_image: Image = Default::default();
    let min_x = image.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = image.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = image.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = image.iter().map(|(_, y)| *y).max().unwrap();
    for y in (min_y - 10)..=(max_y + 10) {
        for x in (min_x - 10)..=(max_x + 10) {
            let i = get_close(&image, x, y, on);
            let v = index[i];
            // print!("{}", if v != on {'#'} else {'.'});
            if v != on {
                new_image.insert((x, y));
            }
            // println!("{} {}: {} {}", x,y,i, v);
        }
        // println!()
    }
    // println!();
    new_image
}

// fn print_image(i: &Image) {
//     for v in i {
//         for v in v {
//             print!("{}", if *v {'#'} else {'.'})
//         }
//         println!()
//     }
// }

fn main() {
    // Por alguna rezon, el algoritmo para laa solucion no va para el ejemplo y viceversa

    let mut lines = include_str!("../../input.txt").lines();
    let index = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|x| x == '#')
        .collect::<Vec<_>>();
    lines.next();
    // let lines = r#"#"#.lines();

    let mut image: Image = lines
        .enumerate()
        .flat_map(|(y, v)| {
            v.trim()
                .chars()
                .enumerate()
                .filter(|(_, v)| *v == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    // print_image(&apply(&index, image, false));

    let mut on = true;
    for _ in 0..50 {
        image = apply(&index, image, on);
        on = !on;
    }
    let count = image.len();
    // println!("{}", image.len());

    println!(">> {}", count);
}
