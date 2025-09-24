import argparse
import os
import subprocess
import sys


def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    default_context = os.path.normpath(os.path.join(script_dir, "..", "admin"))
    default_dockerfile = os.path.join(default_context, "Dockerfile")
    default_dest = os.path.join(script_dir, "web")

    parser = argparse.ArgumentParser(description="Build admin dist via docker buildx and export to web/")
    parser.add_argument("--base-url", default=os.environ.get("VITE_BASE_URL", "/api"))
    parser.add_argument("--oss-region", default=os.environ.get("VITE_OSS_REGION", "cn-guangzhou"))
    parser.add_argument("--oss-bucket", default=os.environ.get("VITE_OSS_BUCKET", "bytefuse"))
    parser.add_argument("--context", default=default_context)
    parser.add_argument("--dockerfile", default=default_dockerfile)
    parser.add_argument("--dest", default=default_dest)
    parser.add_argument("--target", default="export")
    args = parser.parse_args()

    os.makedirs(args.dest, exist_ok=True)
    cmd = [
        "docker",
        "buildx",
        "build",
        "-f",
        args.dockerfile,
        "--target",
        args.target,
        "--build-arg",
        f"VITE_BASE_URL={args.base_url}",
        "--build-arg",
        f"VITE_OSS_REGION={args.oss_region}",
        "--build-arg",
        f"VITE_OSS_BUCKET={args.oss_bucket}",
        "--output",
        f"type=local,dest={args.dest}",
        args.context,
    ]
    print("Running:", " ".join(cmd))
    try:
        subprocess.run(cmd, check=True)
        print(f"Done. Files exported to: {args.dest}")
    except subprocess.CalledProcessError as e:
        print(f"Build failed with exit code {e.returncode}", file=sys.stderr)
        sys.exit(e.returncode)


if __name__ == "__main__":
    main()
