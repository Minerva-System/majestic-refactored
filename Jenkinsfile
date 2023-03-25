pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh '/bin/bash -c "source ~/.cargo/env && cargo build"'
      }
    }

  }
}