use sudo_test::{Command, Env, Group};

use crate::Result;

#[test]
fn sets_primary_group_id() -> Result<()> {
    let gid = 1000;
    let group_name = "rustaceans";
    let env = Env("").group(Group(group_name).id(gid)).build()?;

    let actual = Command::new("su")
        .args(["-g", group_name, "-c", "id -g"])
        .output(&env)?
        .stdout()?
        .parse::<u32>()?;

    assert_eq!(gid, actual);

    Ok(())
}

#[test]
#[ignore = "gh553"]
fn original_primary_group_id_is_lost() -> Result<()> {
    let gid = 1000;
    let group_name = "rustaceans";
    let env = Env("").group(Group(group_name).id(gid)).build()?;

    let actual = Command::new("su")
        .args(["-g", group_name, "-c", "id -G"])
        .output(&env)?
        .stdout()?;

    assert_eq!(gid.to_string(), actual);

    Ok(())
}

#[test]
#[ignore = "gh552"]
fn invoking_user_must_be_root() -> Result<()> {
    let group_name = "rustaceans";
    let invoking_user = "ferris";
    let a_target_user = "ghost";
    let env = Env("")
        .user(invoking_user)
        .user(a_target_user)
        .group(group_name)
        .build()?;

    let target_users = ["root", a_target_user];

    for target_user in target_users {
        let output = Command::new("su")
            .args(["-g", group_name, target_user])
            .as_user(invoking_user)
            .output(&env)?;

        assert!(!output.status().success());
        assert_eq!(Some(1), output.status().code());
        assert_contains!(
            output.stderr(),
            "su: only root can specify alternative groups"
        );
    }

    Ok(())
}
