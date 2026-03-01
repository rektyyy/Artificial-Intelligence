import torch
from torch.profiler import profile, record_function, ProfilerActivity

def level_1():
    with record_function('level1'):
        return torch.randn(100, 50, 1000).sum() + (50 * torch.randn(100, 1000).sum())


def better():
    result = 0
    x = torch.randn((100, 1000))

def run():
    with profile(
        activities=[ProfilerActivity.CPU],
        record_shapes=True,
        profile_memory=True
    ) as prof:
        with record_function("bzdurny_pipeline"):
            level_1()

    print(prof.key_averages().table(
        sort_by="cpu_time_total",
        row_limit=20
    ))


if __name__ == "__main__":
    run()
