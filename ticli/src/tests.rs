use super::Command;

#[test]
fn parsing1() {
    let s1 = "main -t1 sub -Cconfig.txt --test-123=47 -- more arguments";
    let command: Command = s1.into();
    assert_eq!(command.to_string(), s1);
    let s2 = "main -t    1 sub  -C    config.txt --test-123  47 -- more   arguments";
    let command: Command = s2.into();
    assert_eq!(command.to_string(), s1);
}

#[test]
fn parsing2() {
    let s1 = "";
    let command: Command = s1.into();
    assert_eq!(command.to_string(), s1);
    let s2 = "./main test";
    let command: Command = s2.into();
    assert_eq!(command.to_string(), s2);
}

#[test]
fn parsing3() {
    let s1 = "./main --b 2 --a=1 -c 43 -d77 -dsome_more sub1 sub2 --xyz this_is_a_test --hi=Bob sub3 -C file -- args";
    let command: Command = s1.into();
    assert_eq!(command.to_string(), "./main -c43 -d77 -dsome_more --a=1 --b=2 sub1 sub2 --hi=Bob --xyz=this_is_a_test sub3 -Cfile -- args");
}
