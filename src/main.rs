use core::panic;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};

use rand::Rng;
use star::{algorithms::bfs::StandardBFS, data_structures::{bitvec::FastBitvec, graph::{self, Graph}}};

#[derive(Clone, Debug)]
struct VertexCover {
    name: String,
    graph: Graph,
    cover: Vec<usize>,
    probabilities: Vec<(usize, f32)>,
    degrees: Vec<(usize, usize)>,
}

fn main() {
    //rewrite_test_graphs();
    let vertex_covers = read_file("./outputs_test.csv".to_string());

    println!("finished building vertex covers");
    for vc in vertex_covers {
        println!("graph: {}, greedy: {}, nn: {}", vc.name, greedy_vertex_cover_deg(&vc), greedy_vertex_cover_probabilities(&vc));
    }

    /*let f = File::open("./road-germany-osm.mtx").expect("file not found");
    let buf_read = BufReader::new(f);
    let graph = Graph::try_from(buf_read).unwrap();

    for i in 0..1001 {
        println!("iteration: {}", i);
        let walk = generate_random_walk(&graph, 150);
        println!("walk generated");
        let new_graph = graph_from_walk(&walk, &graph);
        println!("graph from walk created");
        new_graph.write_to_file(format!("./new_graphs/road-germany-osm-{}.mtx", i).as_str()).unwrap();
        println!("graph written to file");
        println!("------------------------------------------------------")
    }*/

    


}

fn read_file(path: String) -> Vec<VertexCover> {
    let mut map = HashMap::new();
    let file = fs::read_to_string(path).expect("Unable to read file");

    file.lines().skip(1).for_each(|l| {
        let cols = l.split(',').map(|c| c.trim()).collect::<Vec<&str>>();
        let graph_name = cols[0].split('_').take(1).collect::<Vec<&str>>().join("_");
        let f = File::open(format!("./graphs/{}.txt", graph_name)).expect("file not found");
        let buf_read = BufReader::new(f);
        let graph = Graph::try_from(buf_read).unwrap();
        let degrees = graph
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| (i, e.len()))
            .collect::<Vec<(usize, usize)>>();

        let vertex = cols[0].split('_').nth(1).unwrap().parse::<usize>().unwrap();
        if !map.contains_key(&graph_name) {
            map.insert(
                graph_name.clone(),
                VertexCover {
                    name: graph_name.clone(),
                    graph,
                    cover: Vec::new(),
                    probabilities: Vec::new(),
                    degrees
                },
            );
        }
        let mut vc = map.get(&graph_name).unwrap().clone();

        cols.iter().skip(1).for_each(|c| {
            if c == &"" {
                return;
            }
            if c.parse::<f32>().unwrap().round() == 1.0 {
                vc.cover.push(vertex);
            }
            vc.probabilities.push((vertex, c.parse::<f32>().unwrap()));
        });

        map.insert(graph_name, vc);
    });

    map.iter().map(|(_, v)| v).cloned().collect()
}

fn _rewrite_test_graphs() {
    for file in fs::read_dir("/home/nina/knn/test_graphs/").expect("no such directory") {
        let asd = file.as_ref().unwrap().file_name();
        let file_name = asd.to_str().unwrap();

        let content = fs::read_to_string(format!("/home/nina/knn/test_graphs/{}", file_name)).unwrap();

        fs::write(
            format!("./graphs/{}", file.unwrap().file_name().to_str().unwrap()),
            content
                .lines()
                .map(|l| {
                    let cols = l.split_whitespace().collect::<Vec<&str>>();
                    if cols[0] == "p" {
                        let ret = format!("{}\n", cols[2].parse::<usize>().unwrap());
                        ret.split_whitespace()
                            .next()
                            .unwrap()
                            .parse::<usize>()
                            .unwrap();
                        return ret;
                    }

                    format!(
                        "{} {}\n",
                        cols[0].parse::<usize>().unwrap() - 1,
                        cols[1].parse::<usize>().unwrap() - 1
                    )
                })
                .collect::<String>(),
        )
        .unwrap();
    }
}

fn verify_vertex_cover(vc: &Vec<usize>, graph: &Graph) -> bool {
    let mut g = graph.clone();

    for n in vc {
        for m in g.edges[*n].clone() {
            g.remove_edge((*n, m))
        }
    }

    g.edges.iter().all(|e| e.len() == 0)
}

fn greedy_vertex_cover_deg(vc: &VertexCover) -> usize {
    let mut graph = vc.graph.clone();
    let mut degrees = vc.degrees.clone();
    degrees.sort_by_key(|k| k.1);
    degrees.reverse();
    let mut cover = Vec::new();
    let mut counter = 0;

    while !graph.edges.iter().all(|e| e.len() == 0) {
        cover.push(degrees[counter].0);
        graph.remove_node(degrees[counter].0);
        counter += 1;
    }

    if !verify_vertex_cover(&cover, &vc.graph) {
        panic!("not valid");
    }

    cover.len()
}

fn greedy_vertex_cover_probabilities(vc: &VertexCover) -> usize{
    let mut graph = vc.graph.clone();
    let mut probabilities = vc.probabilities.clone();
    let mut cover = Vec::new();
    probabilities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    probabilities.reverse();
    let mut counter = 0;

    while !graph.edges.iter().all(|e| e.len() == 0) {
        cover.push(probabilities[counter].0);
        graph.remove_node(probabilities[counter].0);
        counter += 1;
    }

    if !verify_vertex_cover(&cover, &vc.graph) {
        panic!("not valid");
    }

    cover.len()
}

fn generate_random_walk(graph: &Graph, length: usize) -> Vec<usize> {
    let mut walk = Vec::new();
    let mut rng = rand::thread_rng();
    walk.push(rng.gen_range(0..graph.edges.len()));

    for _ in 0..length {
        let mut x = rng.gen_range(0..walk.len());
        let mut y = rng.gen_range(0..graph.edges[walk[x] as usize].len());
        while walk.contains(&graph.edges[walk[x] as usize][y]) {
            x = rng.gen_range(0..walk.len());
            y = rng.gen_range(0..graph.edges[walk[x] as usize].len());
        }
        let next = graph.edges[walk[x] as usize][y];
        walk.push(next);
    }

    walk
}

fn graph_from_walk(walk: &Vec<usize>, old_graph: &Graph) -> Graph {
    let mut old_graph = old_graph.clone();

    for i in 0..old_graph.nodes {
        if !walk.contains(&i) {
            old_graph.remove_node(i);
        }
    }
    let mut map = HashMap::new();
    let mut graph = Graph::new_with_nodes(walk.len());

    for i in 0..walk.len() {
        map.insert(walk[i], i);
    }

    for n in walk {
        for m in old_graph.edges[*n].clone() {
            graph.add_edge((map[n], map[&m]));
        }
    }

    assert!(StandardBFS::new(&graph, 0, &mut FastBitvec::new(graph.nodes)).count() == walk.len());

    graph
}
