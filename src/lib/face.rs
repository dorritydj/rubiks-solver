use crate::{Colors, Edge, Corner};

#[derive(Debug)]
pub struct Face<'a> {
    pub color: Colors,
    pub adjacencies: Vec<&'a Colors>,
    // corners: [Corner; 4],
    edges: Vec<Edge>,
}

impl<'a> Face<'a> {
    pub fn new(color: Colors) -> Self {
        let (color, adjacencies) = Colors::connections(color);
        let edges = Self::add_edges(&color, &adjacencies);
        let corners = Self::add_corners(&color, &adjacencies);

        return Self {
            color,
            adjacencies,
            edges,
        };
    }

    fn add_edges(color: &Colors, adjacencies: &Vec<&Colors>) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();

        for &adj in adjacencies {
            let edge = Edge::new(*color, *adj);
            edges.push(edge);
        }

        return edges;
    }

    fn add_corners(color: &Colors, adjacencies: &Vec<&Colors>) -> Vec<Corner> {
        let mut corners: Vec<Corner> = Vec::new();
        
        // TODO:
        // - three pairs of opposites: the face you're on, and the two remaining, A and B
        // - start with A[0]/A[1] can then both be connected to B[0] and B[1]
        // - then just connect the current face with those new connections
        //
        // PROBLEM
        // - this will happen for each face
        // - realistically, we only need one set of these for all corners/edges
        //      - ie, a unique corner will appear three times across all faces
        //      - once for each face
        //      - we should only have one of those
        // - which means we need to build all combinations of corners/edges first
        // - and then feed the appropriate ones into the face
        // - maybe pull this work out into the cube?

        for (index, &adj) in adjacencies.iter().enumerate() {
            // let mut adj_corners = Vec::new();
            // let next_index = (index + 1) % 4;
            
            // println!("{:?}", adjacencies);
            // // this isn't evaluating all possible permutations
            // // just whatever is next to the current in the list
            // if Colors::is_opposite_side(adj, adjacencies[next_index]) {
            //     continue;
            // }

            // adj_corners.push(*adj);
            // adj_corners.push(*adjacencies[next_index]);
        
            // let corner = Corner::new(*color, adj_corners);
            // println!("{:?}", corner);
        }

        return corners; 
    }
}
