struct MerkleTree {
    length: usize,
    tree: Vec<u32>,
    root: u32,
    steps: Vec<usize>
}

impl MerkleTree {
    pub fn create_tree(nodes:  Vec<u32>) -> MerkleTree {
        let mut working_vec: Vec<u32> = nodes.clone();
        let mut working_node_total = nodes.len();
        let mut steps: Vec<usize> = Vec::new();
        
        if nodes.len() % 2 == 1 {
            working_vec.push(working_vec[working_node_total-1]);
            working_node_total+=1;
        }
        let initial_length = working_node_total;

        // go through vector, add numbers together into new vector check it is even
        // add the new vector to the end
        // keep going until there is only 1 and that is the root node
        let mut startpoint = 0;
        let mut vec_to_add: Vec<u32> = Vec::new();
        loop {
            // make sure tree is even
            if working_node_total % 2 == 1 {
                working_vec.push(working_vec[working_node_total-1]);
            }
            
            for index in (startpoint .. working_vec.len()).step_by(2) {
                vec_to_add.push(working_vec[index] + working_vec[index+1]); // todo make this a hash
            }

            startpoint = working_vec.len();
            working_vec.append(&mut vec_to_add.clone());
            working_node_total = working_vec.len();

            if (vec_to_add.len()) / 2 == 1 {
                steps.push((vec_to_add.len()+1).try_into().unwrap());
            }
            else {
                steps.push((vec_to_add.len()).try_into().unwrap());
            }

            if vec_to_add.len() == 1 {
                return MerkleTree {
                    length: initial_length,
                    tree: working_vec.clone(),
                    root: working_vec[working_vec.len()-1],
                    steps: steps
                }
            }
            vec_to_add.clear();
        }
    }
    
    pub fn check_value(test :u32, tree: &MerkleTree) -> bool {
        let steps = &tree.steps;
        let mut end_point = tree.length;
        let nodes = &tree.tree;
        let mut hunted = test;
        let mut startpoint = 0;
        let mut step_number = 0;
        loop {
            println!("checking from index {startpoint} to {end_point}");

            let mut index = 0;    
            while startpoint + index < end_point {
                println!("checking index {index} for {hunted}");
                println!("checking from index {startpoint} to {end_point}");
                let t = &nodes[index];
                println!("found {t}");

                if hunted == tree.root {
                    return true;
                }

                if nodes[startpoint + index] == hunted {
                    if index % 2 == 1 {
                        // it is on the right hand side
                        hunted += nodes[startpoint + index-1]; // todo make this a hash
                    }
                    else {
                        // it is on the left hand side
                        hunted += nodes[startpoint + index+1]; // todo make this a hash
                    }
                    startpoint = end_point;
                    end_point = end_point + steps[step_number];
                    step_number = step_number + 1;
                    index = 0;
                    continue;
                }

                index = index + 1;
            }
            return false;        
        }
    }
    


    pub fn get_proof(test :u32, tree: &MerkleTree) -> Option<Vec<u32>> {
        let steps = &tree.steps;
        let mut end_point = tree.length;
        let nodes = &tree.tree;
        let mut hunted = test;
        let mut startpoint = 0;
        let mut step_number = 0;
        let mut proof: Vec<u32> = Vec::new();
        loop {
            println!("checking from index {startpoint} to {end_point}");
    
            let mut index = 0;    
            while startpoint + index < end_point {
                println!("checking index {index} for {hunted}");
                println!("checking from index {startpoint} to {end_point}");
                let t = &nodes[index];
                println!("found {t}");
    
                if hunted == tree.root {
                    proof.push(hunted);
                    return Some(proof);
                }
    
                if nodes[startpoint + index] == hunted {
                    proof.push(hunted);
                    if index % 2 == 1 {
                        // it is on the right hand side
                        hunted += nodes[startpoint + index - 1]; // todo make this a hash
                        proof.push(nodes[startpoint + index - 1]);
                    }
                    else {
                        // it is on the left hand side
                        hunted += nodes[startpoint + index + 1]; // todo make this a hash
                        proof.push(nodes[startpoint + index + 1]);
                    }
                    startpoint = end_point;
                    end_point = end_point + steps[step_number];
                    step_number = step_number + 1;
                    index = 0;
                    continue;
                }
    
                index = index + 1;
            }
            return None;        
        }
    }
}



fn main() {
    let test = vec![2,3,4,5,6];
    let tree = MerkleTree::create_tree(test);
    assert_eq!(MerkleTree::check_value(2, &tree), true);
    assert_eq!(MerkleTree::check_value(6, &tree), true);
    assert_eq!(MerkleTree::check_value(900, &tree), false);
    assert_eq!(MerkleTree::get_proof(2, &tree).unwrap()[1], 3);

    let bigger_test = vec![2,3,4,5,6,8,9,10,11,12,13,14,15,16,17,18];
    let bigger_tree = MerkleTree::create_tree(bigger_test);
    assert_eq!(MerkleTree::check_value(2, &bigger_tree), true);
    assert_eq!(MerkleTree::check_value(10, &bigger_tree), true);
    assert_eq!(MerkleTree::check_value(18, &bigger_tree), true);
    assert_eq!(MerkleTree::check_value(900, &bigger_tree), false);
    assert_eq!(MerkleTree::get_proof(900, &tree), None);
    assert_eq!(MerkleTree::get_proof(1, &tree).unwrap()[1], 19);

    let small_test = vec![2];
    let smaller_tree = MerkleTree::create_tree(small_test);
    assert_eq!(MerkleTree::check_value(2, &smaller_tree), true);
    assert_eq!(MerkleTree::check_value(10, &smaller_tree), false);    
}


