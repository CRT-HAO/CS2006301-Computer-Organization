import os
import subprocess

from decimal import Decimal, getcontext

import click


# Set decimal point accuracy
getcontext().prec = 6

g_gem5_executable: str = "../build/RISCV/gem5.opt"
g_gem5_config: str = "../configs/deprecated/example/se.py"
g_test_program: str = ""


def extract_miss_rate(stats_file: str) -> tuple[Decimal, Decimal]:
    def get_value_from_stats(keyword) -> str | None:
        result = subprocess.run(
            ["grep", keyword, stats_file], capture_output=True, text=True
        )
        return result.stdout.split()[1] if result.stdout else None

    dcache_misses = get_value_from_stats("system.cpu.dcache.overallMisses::total")
    icache_misses = get_value_from_stats("system.cpu.icache.overallMisses::total")
    dcache_accesses = get_value_from_stats("system.cpu.dcache.overallAccesses::total")
    icache_accesses = get_value_from_stats("system.cpu.icache.overallAccesses::total")

    if not all([dcache_misses, icache_misses, dcache_accesses, icache_accesses]):
        raise ValueError(
            f"Error: Failed to extract miss or access rates from {stats_file}."
        )

    if any(
        v is None
        for v in [dcache_misses, icache_misses, dcache_accesses, icache_accesses]
    ):
        raise ValueError(
            f"Error: Failed to extract miss or access rates from {stats_file}."
        )

    dcache_miss_rate = Decimal(dcache_misses) / Decimal(dcache_accesses)  # type: ignore
    icache_miss_rate = Decimal(icache_misses) / Decimal(icache_accesses)  # type: ignore

    return dcache_miss_rate, icache_miss_rate


def calculate_growth_rate(new_rate, base_rate):
    return ((new_rate - base_rate) / base_rate * 100).quantize(Decimal("0.01"))


def run_simulation(
    l1i_size: str | int,
    l1i_assoc: str | int,
    l1d_size: str | int,
    l1d_assoc: str | int,
    cacheline_size: str | int,
    output_dir: str,
) -> None:
    global g_gem5_executable, g_gem5_config, g_test_program

    os.makedirs(output_dir, exist_ok=True)
    command = [
        g_gem5_executable,
        g_gem5_config,
        "--caches",
        f"--l1i_size={l1i_size}",
        f"--l1i_assoc={l1i_assoc}",
        f"--l1d_size={l1d_size}",
        f"--l1d_assoc={l1d_assoc}",
        f"--cacheline_size={cacheline_size}",
        "-c",
        g_test_program,
    ]
    with open(f"{output_dir}/gem5.log", "w") as log_file:
        subprocess.run(command, stdout=log_file, stderr=subprocess.STDOUT)
    subprocess.run(["cp", "m5out/stats.txt", f"{output_dir}/stats.txt"])
    subprocess.run(["cp", "m5out/config.ini", f"{output_dir}/config.ini"])


@click.command()
@click.option(
    "-g",
    "--gem5_executable",
    help="gem5 executable path",
    required=True,
    type=str,
)
@click.option(
    "-c",
    "--gem5_config",
    help="gem5 config script",
    required=True,
    type=str,
)
@click.option(
    "-p",
    "--test_program",
    help="test program executable",
    required=True,
    type=str,
)
def main(gem5_executable: str, gem5_config: str, test_program: str) -> None:
    global g_gem5_executable, g_gem5_config, g_test_program
    g_gem5_executable = gem5_executable
    g_gem5_config = gem5_config
    g_test_program = test_program

    base_config = ("8kB", 2, "8kB", 2, 64, "m5out/base")
    run_simulation(*base_config)

    cacheline_sizes = [8, 16, 32, 128, 256]
    for cacheline_size in cacheline_sizes:
        run_simulation(
            "8kB", 2, "8kB", 2, cacheline_size, f"m5out/cacheline_size={cacheline_size}"
        )

    base_dcache_miss_rate, base_icache_miss_rate = extract_miss_rate(
        "m5out/base/stats.txt"
    )

    for cacheline_size in cacheline_sizes:
        dcache_miss_rate, icache_miss_rate = extract_miss_rate(
            f"m5out/cacheline_size={cacheline_size}/stats.txt"
        )
        dcache_miss_growth = calculate_growth_rate(
            dcache_miss_rate, base_dcache_miss_rate
        )
        icache_miss_growth = calculate_growth_rate(
            icache_miss_rate, base_icache_miss_rate
        )

        print(f"Cacheline Size: {cacheline_size}")
        print(f"Data Cache Miss Rate Growth: {dcache_miss_growth}%")
        print(f"Instruction Cache Miss Rate Growth: {icache_miss_growth}%")
        print("\n")


if __name__ == "__main__":
    main()
