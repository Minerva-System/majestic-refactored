podTemplate(containers: [
	containerTemplate(
		name: 'rust',
		image: 'rust:1.68.1-alpine',
		command: 'sleep',
		args: '30d'
	)
]) {
	node(POD_LABEL) {
		stage('Rust project build') {
			container('rust-builder') {
				stage('Clone project') {
					git 'https://github.com/Minerva-System/majestic-refactored'
					sh 'cd majestic-refactored'
				}
				stage('Build') {
					sh 'cargo build --release'
				}
			}
		}
	}
}
