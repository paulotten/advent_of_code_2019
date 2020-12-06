mod data;

fn main() {
    let data = data::get_data();
    let data: Vec<char> = data.chars().collect();
    let layers = get_layers(&data);

    puzzle1(layers);
}

fn puzzle1(layers: Vec<String>) {
    let mut min0 = SIZE+1;
    let mut min0_layer = 0;

    for i in 0..layers.len() {
        let occurances = layers[i].matches('0').count();

        if occurances < min0 {
            min0 = occurances;
            min0_layer = i;
        }
    }

    let c1 = layers[min0_layer].matches('1').count();
    let c2 = layers[min0_layer].matches('2').count();
    println!("{}", c1 * c2);
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

fn get_layers(data: &[char]) -> Vec<String> {
    let mut layers = vec!();
    let mut position = 0;

    while position + SIZE <= data.len() {
        layers.push(data[position..position+SIZE].iter().collect());
        position += SIZE;
    }

    layers
}