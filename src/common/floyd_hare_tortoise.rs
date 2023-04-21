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
    floyd_hare_tortoise_with_cmp(gen, step, |a, b| a == b)
}

/**
 * Floyd's hare and tortoise algorithm
 * Input: gen - function that generates initial value
 * (maybe by cloning some value)
 * step - function that mutates value, moving it one step forward
 * eq - if special compare (not == of PartialEq) is needed
 *
 * Return: (lambda, mu)
 * lambda - length of cycle
 * mu - index of the first element in cycle
 */

pub fn floyd_hare_tortoise_with_cmp<Type, Gen, Step, Eq>(
    gen: Gen,
    step: Step,
    eq: Eq,
) -> (usize, usize)
where
    Type: Clone,
    Gen: Fn() -> Type,
    Step: Fn(&mut Type),
    Eq: Fn(&Type, &Type) -> bool,
{
    let mut hare = gen();
    let mut tortoise = hare.clone();
    loop {
        step(&mut hare);
        step(&mut hare);
        step(&mut tortoise);
        if eq(&hare, &tortoise) {
            break;
        }
    }
    let mut mu = 0;
    let mut tortoise = gen();
    while !eq(&hare, &tortoise) {
        step(&mut hare);
        step(&mut tortoise);
        mu += 1;
    }
    let mut lam = 1;
    let mut hare = tortoise.clone();
    loop {
        step(&mut hare);
        if eq(&hare, &tortoise) {
            break;
        }
        lam += 1;
    }
    (lam, mu)
}
