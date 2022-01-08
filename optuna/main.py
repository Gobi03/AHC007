import optuna
import os
import pwn
import sys

pwn.context.log_level = "error"

def objective(trial):
    agree_line = trial.suggest_int("agree_line", 0, 47)

    test_num = '0000'
    command = f'../target/release/a'
    s = pwn.process(command)

    print("hoge")


    while True:
        l = s.recvline().decode()[:-1]
        print(l)
        if l.startswith("cost: "):
            s.close()
            return int(l[5:])

# main
os.system("cargo build --release")

study = optuna.create_study(
    study_name="ahc007",
    storage="sqlite:///db.sqlite3",
    load_if_exists=True,
    direction="minimize")

study.optimize(objective, n_trials=10)
