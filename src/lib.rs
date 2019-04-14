pub mod neural_net;
pub mod neuron;
use neuron::*;

#[cfg(test)]
mod test {
    use super::*;

    fn threshold(x: f32) -> f32 {
        if x >= 1. { 1. } else { 0. }
    }

    #[test]
    fn gate() {
        let mut n1 = Neuron::new(Box::new(threshold), NeuronType::Output);
        let mut n2 = Neuron::new(Box::new(threshold), NeuronType::Input);
        let mut n3 = Neuron::new(Box::new(threshold), NeuronType::Input);
        let mut n4 = Neuron::new(Box::new(threshold), NeuronType::Input);

        n1.link(&n2).with_weight(1.);
        n1.links(&vec![&n3,&n4]).with_weights(vec![1.,0.]);
        n1.links(&vec![&n3,&n4]).with_weights(vec![1.,0.]);
    }
}


