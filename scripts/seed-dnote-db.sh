#!/bin/bash

# Check for --no-confirm flag
NO_CONFIRM=false
for arg in "$@"; do
  if [ "$arg" == "--no-confirm" ]; then
    NO_CONFIRM=true
    break
  fi
done

# Prompt for user confirmation
if [ "$NO_CONFIRM" == false ]; then
  read -p "This script will add new mock books and notes to your dnote db. Do you want to proceed? (y/n): " confirm
  if [[ "$confirm" != "y" ]]; then
    echo "Operation cancelled."
    exit 0
  fi
fi

# JSON Data embedded in the script
JSON_DATA=$(cat <<EOF
[
    {
        "book": "RustProgramming",
        "notes": [
            "Introduction to Rust: Rust is a systems programming language focused on safety and performance.",
            "Ownership and Borrowing: Understanding ownership in Rust is key to managing memory safety without garbage collection.",
            "Concurrency in Rust: Rust makes it easier to write safe and concurrent programs.",
            "Crates and Modules: Organize your Rust code with crates and modules."
        ]
    },
    {
        "book": "DockerEssentials",
        "notes": [
            "Introduction to Docker: Docker enables containerization which can significantly improve development workflows.",
            "Basic Docker Commands: Learn about docker build, run, and other essential commands to manage containers.",
            "Docker Compose: Use docker-compose to manage multi-container Docker applications.",
            "Docker Networking: Understand how Docker manages networking in containers."
        ]
    },
    {
        "book": "DevOpsPractices",
        "notes": [
            "Continuous Integration: CI involves automatically building and testing code changes frequently.",
            "Continuous Delivery: CD ensures that code changes are automatically prepared for a release to production.",
            "Infrastructure as Code: Manage your infrastructure with code using tools like Terraform or Ansible.",
            "Monitoring and Logging: Ensure your applications are monitored and logs are collected."
        ]
    },
    {
        "book": "PythonProgramming",
        "notes": [
            "Introduction to Python: Python is a versatile language suitable for many tasks.",
            "Python Data Types: Familiarize yourself with Python's built-in data types.",
            "Working with Lists: Lists are one of the most powerful tools in Python.",
            "Object-Oriented Programming in Python: Learn about classes and objects."
        ]
    },
    {
        "book": "Kubernetes",
        "notes": [
            "Introduction to Kubernetes: Kubernetes helps manage containerized applications at scale.",
            "K8s Pods: Pods are the smallest deployable units of computing in Kubernetes.",
            "Services and Networking: Learn how services enable communication between different pods.",
            "K8s Volumes: Understand how Kubernetes manages storage for containers."
        ]
    },
    {
        "book": "MachineLearning",
        "notes": [
            "Introduction to ML: Machine Learning is a subset of AI that focuses on predictive modeling.",
            "Supervised Learning: Understand the concepts of supervised learning and its applications.",
            "Unsupervised Learning: Learn how unsupervised learning works and its use cases.",
            "Deep Learning: Dive into deep learning and neural networks."
        ]
    },
    {
        "book": "CloudComputing",
        "notes": [
            "Introduction to Cloud: Cloud Computing provides on-demand computing resources.",
            "AWS Overview: Amazon Web Services is a popular cloud service provider.",
            "Azure Basics: Learn about Microsoft cloud service, Azure.",
            "Google Cloud Platform: Understand the key features of GCP."
        ]
    },
    {
        "book": "WebDevelopment",
        "notes": [
            "Introduction to Web Development: Web development involves creating websites and web applications.",
            "HTML & CSS: Learn the basics of HTML and CSS for structuring and styling web pages.",
            "JavaScript Fundamentals: JavaScript is essential for interactive web pages.",
            "Front-End Frameworks: Get to know popular front-end frameworks like React and Vue."
        ]
    },
    {
        "book": "DatabaseManagement",
        "notes": [
            "Introduction to Databases: Databases store and manage data.",
            "SQL Basics: Learn about Structured Query Language for querying databases.",
            "NoSQL Databases: Understand the principles of NoSQL databases like MongoDB.",
            "Database Indexing: Optimize database performance with indexing."
        ]
    },
    {
        "book": "SoftwareEngineering",
        "notes": [
            "Introduction to Software Engineering: Software engineering is the application of engineering principles to software development.",
            "Agile Methodologies: Learn about Agile methodologies for iterative development.",
            "Design Patterns: Familiarize yourself with common software design patterns.",
            "Testing and QA: Ensure software quality with proper testing and quality assurance practices."
        ]
    }
]
EOF
)

# Install jq for JSON parsing if not already installed
if ! command -v jq &> /dev/null; then
    echo "jq could not be found, installing..."
    apt-get update && apt-get install -y jq
fi

# Debug: Output the JSON_DATA to ensure it's parsed correctly
echo "$JSON_DATA" | jq '.' || { 
    echo "Invalid JSON data"; 
    exit 1; 
}

# Read JSON data and create books and notes
echo "$JSON_DATA" | jq -c '.[]' | while IFS= read -r BOOK; do
    BOOK_NAME=$(echo "$BOOK" | jq -r '.book')
    echo "Processing book: $BOOK_NAME..."
    echo "$BOOK" | jq -c '.notes[]' | while IFS= read -r NOTE; do
        NOTE_CONTENT=$(echo "$NOTE" | jq -r '.')
        echo "Adding note: $NOTE_CONTENT"
        dnote add "$BOOK_NAME" -c "$NOTE_CONTENT"
    done
done

echo "Mock Books and notes have been created."
