/// Utils for converting loss to fitness value. Can be user defined in future, write with dependency inversion in mind.

pub fn l1_loss_to_reciprocal_fitness(loss: f64) -> f64 {
    return 1.0 / (1.0 + loss)
}