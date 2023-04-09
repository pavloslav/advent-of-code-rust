/**
 * Floyd's hare and tortoise algorithm
 * Input: gen - function that generates initial value
 * (maybe by cloning some value)
 * step - function that mutates value, moving it one step forward
 *
 * Return: (lambda, mu)
 * lambda - length of cycle
 * mu - index of the first element in cycle
 */

pub fn floyd_hare_tortoise<Type, Gen, Step>(
    gen: Gen,
    step: Step,
) -> (usize, usize)
where
    Gen: Fn() -> Type,
    Step: Fn(&mut Type),
    Type: PartialEq + Clone,
{
    let mut hare = gen();
    let mut tortoise = gen();
    loop {
        step(&mut hare);
        step(&mut hare);
        step(&mut tortoise);
        if hare == tortoise {
            break;
        }
    }
    let mut mu = 0;
    let mut tortoise = gen();
    while tortoise != hare {
        step(&mut hare);
        step(&mut tortoise);
        mu += 1;
    }
    let mut lam = 1;
    let mut hare = tortoise.clone();
    loop {
        step(&mut hare);
        if hare == tortoise {
            break;
        }
        lam += 1;
    }
    (lam, mu)
}
