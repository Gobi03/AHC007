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
    s.read(f'tools/in/{test_num}.txt')

    print("hoge")

    l = s.recvline().decode()[:-1]

    print("fuga")

    # score = float(s.recvline().decode()[:-1])/test_num*50
    s.close()
    
    return score

study = optuna.create_study(
    study_name="ahc007",
    storage="sqlite:///db.sqlite3",
    load_if_exists=True,
    direction="maximize")

study.optimize(objective, n_trials=100)
