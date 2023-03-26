podTemplate(containers: [
    containerTemplate(
        name: 'rust',
        image: 'rust:1.68.1',
        command: 'sleep',
        args: '30d'
    )
]) {
    node(POD_LABEL) {
        stage('Geração de Versão') {
            container('rust') {
                stage('Clonar repositório') {
                    git 'https://github.com/Minerva-System/majestic-refactored'
                }
		
                stage('Compilação') {
                    sh 'cargo build --release'
                }

		stage('Empacotamento') {
		    sh '''
                        cp target/release/majestic-refactored majestic
                        MAJESTIC_VERSION=`grep version Cargo.toml | awk '{print $3}' | tr -d '"'`
                        tar -czvf "majestic-${MAJESTIC_VERSION}.tar.gz" majestic
                        rm majestic
                    '''
		}
		
		archiveArtifacts artifacts: '*.tar.gz',
		    allowEmptyArchive: false,
		    fingerprint: true,
		    onlyIfSuccessful: true
            }
        }
    }
}
