import optuna
import os
import pwn

pwn.context.log_level = "error"

def objective(trial):
    adopt_line = trial.suggest_int("adopt_line", 1, 47)

    test_num = '0000'
    command = '../target/release/a'
    with pwn.process(command, env={'ADOPT_LINE': str(adopt_line)}) as s:
        with open(f'./tools/in/{test_num}.txt', ('rb')) as f:
            data = f.read()
            s.send(data)

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

study.optimize(objective, n_trials=100)

print(f"\nbest_params: {study.best_params}, best_value: {study.best_value}")
