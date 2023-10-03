use sha256::digest;

struct MerkleTree {
    length: u8,
    tree: Vec<u32>,
    hash_tree: Vec<String>,
    root: u32,
    hashroot: String,
    steps: Vec<u8>
}

impl MerkleTree {
    pub fn create_tree(nodes:  Vec<u32>) -> MerkleTree {
        let mut working_vec: Vec<u32> = nodes.clone();
        let mut working_node_total = nodes.len();
        let mut steps: Vec<u8> = Vec::new();
        
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
                    length: initial_length as u8,
                    tree: working_vec.clone(),
                    root: working_vec[working_vec.len()-1],
                    steps: steps,
                    hash_tree: Vec::new(),
                    hashroot: String::new(),
                }
            }
            vec_to_add.clear();
        }
    }
    
    pub fn create_hash_tree(nodes:  Vec<String>) -> MerkleTree {
        let mut working_vec: Vec<String> = nodes.clone();
        let mut working_node_total: usize = nodes.len();
        let mut steps: Vec<u8> = Vec::new();
        
        if nodes.len() % 2 == 1 {
            working_vec.push(working_vec[working_node_total-1].clone());
            working_node_total+=1;
        }
        let initial_length = working_node_total;

        // go through vector, add numbers together into new vector check it is even
        // add the new vector to the end
        // keep going until there is only 1 and that is the root node
        let mut startpoint = 0;
        let mut vec_to_add: Vec<String> = Vec::new();
        loop {
            // make sure tree is even
            if working_node_total % 2 == 1 {
                working_vec.push(working_vec.last().unwrap().clone());
            }
            
            for index in (startpoint .. working_vec.len()).step_by(2) {
                vec_to_add.push(digest(working_vec[index].clone() + &working_vec[index+1])); 
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
                    length: initial_length as u8,
                    tree: Vec::new(),
                    root: 0,
                    hashroot: working_vec.last().unwrap().clone(),
                    steps: steps,
                    hash_tree: working_vec.clone(),
                }
            }
            vec_to_add.clear();
        }
    }
    

    pub fn check_value(test :u32, tree: &MerkleTree) -> bool {
        let steps = &tree.steps;
        let mut end_point = tree.length as usize;
        let nodes = &tree.tree;
        let mut hunted = test;
        let mut startpoint:usize = 0;
        let mut step_number:usize = 0;
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
                    end_point = end_point + steps[step_number] as usize;
                    step_number = step_number + 1;
                    index = 0;
                    continue;
                }

                index = index + 1;
            }
            return false;        
        }
    }
    
    // todo - change this to work with hashes
    pub fn check_hash_value(test :String, tree: &MerkleTree) -> bool {
        let steps = &tree.steps;
        let mut end_point = tree.length as usize;
        let nodes = &tree.hash_tree;
        let mut hunted = test;
        let mut startpoint = 0;
        let mut step_number = 0;
        loop {
            println!("checking from index {startpoint} to {end_point}");

            let mut index: usize = 0;    
            while startpoint + index < end_point {
                println!("checking index {index} for {hunted}");
                println!("checking from index {startpoint} to {end_point}");
                let t = &nodes[index];
                println!("found {t}");

                if hunted.eq(&tree.hashroot) {
                    return true;
                }

                if nodes[startpoint + index] == hunted {
                    if index % 2 == 1 {
                        // it is on the right hand side
                        hunted = digest(nodes[startpoint + index - 1].clone() + &nodes[startpoint + index]) ;
                    }
                    else {
                        // it is on the left hand side
                        hunted = digest(nodes[startpoint + index].clone() + &nodes[startpoint + index + 1]);
                    }
                    startpoint = end_point;
                    end_point = end_point + steps[step_number] as usize;
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
        let mut end_point: usize = tree.length as usize;
        let nodes = &tree.tree;
        let mut hunted = test;
        let mut startpoint: usize = 0;
        let mut step_number:usize = 0;
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
                    end_point = end_point + steps[step_number] as usize;
                    step_number = step_number + 1;
                    index = 0;
                    continue;
                }
    
                index = index + 1;
            }
            return None;        
        }
    }

    pub fn get_hash_proof(test :String, tree: &MerkleTree) -> Option<Vec<String>> {
        let steps = &tree.steps;
        let mut end_point: usize = tree.length as usize;
        let nodes: &Vec<String> = &tree.hash_tree;
        let mut hunted: String = test;
        let mut startpoint: usize = 0;
        let mut step_number = 0;
        let mut proof: Vec<String> = Vec::new();
        loop {
            println!("checking from index {startpoint} to {end_point}");
    
            let mut index = 0;    
            while startpoint + index < end_point {
                println!("checking index {index} for {hunted}");
                println!("checking from index {startpoint} to {end_point}");
                let t = &nodes[index];
                println!("found {t}");
    
                if hunted == tree.hashroot {
                    proof.push(hunted);
                    return Some(proof);
                }
    
                if nodes[startpoint + index] == hunted {
                    proof.push(hunted);
                    if index % 2 == 1 {
                        // it is on the right hand side
                        //hunted += nodes[startpoint + index - 1]; // todo make this a hash
                        // on the right hand side means we concatenate it to the end of the LHS
                        hunted = digest(nodes[startpoint + index - 1].clone() + &nodes[startpoint + index]) ;
                        proof.push(nodes[startpoint + index - 1].clone());
                    }
                    else {
                        // it is on the left hand side
                        //hunted += nodes[startpoint + index + 1]; // todo make this a hash
                        // on the left hand side means we place it on the beginning  of the LHS
                        hunted = digest(nodes[startpoint + index].clone() + &nodes[startpoint + index + 1]);
                        proof.push(nodes[startpoint + index + 1].clone());
                    }
                    startpoint = end_point;
                    end_point = end_point + steps[step_number] as usize;
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

    let hash_test_start = vec!["2","3","4","5","6"];
    let mut hash_test_end: Vec<String>= Vec::new();
    for nodes in hash_test_start {
        hash_test_end.push(digest(nodes));
    }
    let hash_tree = MerkleTree::create_hash_tree(hash_test_end);
 
    assert_eq!(hash_tree.hash_tree[0], digest("2"));
    let d2 = digest("2");
    let d3 = digest("3");
    let d4 = digest("4");
    let d5 = digest("5");
    let d6 = digest("6");

    let d23 = digest(d2.clone()+&d3);
    let d45 = digest(d4.clone()+&d5);
    let d66 = digest(d6.clone()+&d6);

    let d59 = digest(d23.clone()+&d45);
    let d6666 = digest(d66.clone()+&d66);
    let d596666 = digest(d59.clone()+&d6666);

    assert_eq!(hash_tree.hash_tree[0],d2);
    assert_eq!(hash_tree.hash_tree[1],d3);
    assert_eq!(hash_tree.hash_tree[2],d4);
    assert_eq!(hash_tree.hash_tree[3],d5);
    assert_eq!(hash_tree.hash_tree[4],d6);
    assert_eq!(hash_tree.hash_tree[5],d6);

    assert_eq!(hash_tree.hash_tree[6],d23);
    assert_eq!(hash_tree.hash_tree[7],d45);
    assert_eq!(hash_tree.hash_tree[8],d66);
    assert_eq!(hash_tree.hash_tree[9],d66);
    
    assert_eq!(hash_tree.hash_tree[10],d59);    
    assert_eq!(hash_tree.hash_tree[11],d6666);
    assert_eq!(hash_tree.hash_tree[12],d596666);

    let hash_proof = MerkleTree::get_hash_proof(digest("2"), &hash_tree).unwrap();
    let test_proof = vec![d2,d3,d23,d45,d59,d6666,d596666.clone()];
    assert!(hash_proof.eq(&test_proof));

    assert!(MerkleTree::check_hash_value(digest("2"), &hash_tree));
    assert!(MerkleTree::check_hash_value(d596666, &hash_tree));
    assert!(MerkleTree::check_hash_value("Sir Not Appearing in this Hash".to_string(), &hash_tree) == false);

    let bigger_test = vec![2,3,4,5,6,8,9,10,11,12,13,14,15,16,17,18];
    let bigger_tree = MerkleTree::create_tree(bigger_test);
    assert_eq!(MerkleTree::check_value(2, &bigger_tree), true);
    assert_eq!(MerkleTree::check_value(10, &bigger_tree), true);
    assert_eq!(MerkleTree::check_value(18, &bigger_tree), true);
    assert_eq!(MerkleTree::check_value(900, &bigger_tree), false);
    assert_eq!(MerkleTree::get_proof(900, &tree), None);
    
    let small_test = vec![2];
    let smaller_tree = MerkleTree::create_tree(small_test);
    assert_eq!(MerkleTree::check_value(2, &smaller_tree), true);
    assert_eq!(MerkleTree::check_value(10, &smaller_tree), false);    
}


