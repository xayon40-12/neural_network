
/* Types of neurons */
pub enum NeuronType {
    Input,
    Output,
    Hidden
}

/* Scinapse correspond to the connections between neurons: n is the linked neuron and w
 * correspond to the weight of the connection */
pub struct Scinapse<'a> {
    n: &'a Neuron<'a>,
    w: f32
}

impl<'a> Scinapse<'a> {
    pub fn new(n: &'a Neuron) -> Scinapse<'a> {
        Scinapse { n, w: 0. }
    }

    pub fn with_weight(&mut self, weight: f32) {
        self.w = weight;
    }
}

/* same as Scinapse except here n is the linking neuron and w is a reference to the weight of
 * the corresponding Scinapse */
struct BScinapse<'a> {
    n: &'a Neuron<'a>,
    w: &'a f32
}

pub struct Scinapses<'a,'b> {
    scinapses: &'a mut [Scinapse<'b>]
}

impl<'a,'b> Scinapses<'a,'b> {
    pub fn with_weights(&mut self, weights: Vec<f32>) {
        if self.scinapses.len() != weights.len() { panic!("Sizes mismatch in 'with_weights()'."); }

        for i in 0..weights.len() {
            self.scinapses[i].w = weights[i];
        }
    }
}

/* structure of a Neuron */
pub struct Neuron<'a> {
    linked_to: Vec<Scinapse<'a>>,
    linked_by: Vec<BScinapse<'a>>,
    dist: Option<u32>,
    old_value: f32,
    ntype: NeuronType,
    computed: bool,
    transfer: Box<Fn(f32) -> f32>
}

impl<'a> Neuron<'a> {
    pub fn new(transfer: Box<Fn(f32)->f32>, ntype: NeuronType) -> Neuron<'a> {
        Neuron {
            linked_to: Vec::new(), 
            linked_by: Vec::new(), 
            dist: match  ntype { NeuronType::Output => Some(0), _ => None },
            old_value: 0.,
            ntype,
            computed: false,
            transfer
        }
    }

    pub fn link(& mut self, n: &'a Neuron) -> &mut Scinapse<'a> {
        self.linked_to.push(Scinapse{ n, w: 0. });
        if let Some(s) = self.linked_to.last_mut() {
            s
        } else {
            panic!()
        }
    }

    pub fn links<'b>(&'b mut self, ns: &Vec<&'a Neuron>) -> Scinapses<'b,'a> {
        for n in ns {
            self.linked_to.push(Scinapse{ n, w: 0. });
        }
        let l = self.linked_to.len();
        Scinapses { scinapses: &mut self.linked_to[(l-ns.len())..l] }
    }

}

