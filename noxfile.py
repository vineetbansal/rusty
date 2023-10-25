import nox


@nox.session()
def tests(session: nox.Session) -> None:
    session.install("-e.[dev]")
    session.run("pytest")

@nox.session()
def dev(session: nox.Session) -> None:
    session.install("maturin")
    session.run("maturin", "develop", "--extras=dev")
    session.run("pytest")
