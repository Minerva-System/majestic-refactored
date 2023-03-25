pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        sh 'source ~/.cargo/env && cargo build'
      }
    }

  }
}