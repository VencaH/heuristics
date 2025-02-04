def tests = [:]
def test_name = "";

pipeline {
    agent any


    stages {
        stage('Git Checkout') {
             steps {
                script {
                    checkout scm
                }
            }
        }
        stage('Get tests') {
            steps{
                    script {
                    tests = sh(script: 'nix develop --command bash -c "cargo test -q -- --list"', returnStdout: true ).tokenize("\n")
                    echo "Tests: ${tests}"
                    }
            }
        }
        
        stage('Run tests') {
            steps {
                script {
                    tests.each { test_text ->
                        test_name = test_text.split(": ")[0]
                        stage("Run test ${test_name}") {
                            script {
                                def test_result = sh(script:"nix develop --command bash -c \"cargo test ${test_name} -q -- --exact\"", returnStdout: true)
                                echo "Test result: ${test_result}"
                            }
                        }
                    }
                }
            }
        }
    }
    post {
        always {
            sh 'nix-store --gc'
        }
    }
}
