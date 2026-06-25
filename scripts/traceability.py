import subprocess
import sys

def main():
    print("=== Traceability Report ===")
    print("Delegating to unified Rust Traceability Engine...")
    
    # Run the rust binary
    result = subprocess.run(["cargo", "run", "--bin", "traceability_cli"], capture_output=True, text=True)
    
    print(result.stdout)
    if result.stderr:
        print(result.stderr, file=sys.stderr)
        
    if result.returncode != 0:
        sys.exit(result.returncode)

if __name__ == "__main__":
    main()
