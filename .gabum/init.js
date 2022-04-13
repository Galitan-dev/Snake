const { spawn } = require('child_process');
const { writeFileSync, readFileSync } = require('fs');
const PATH = require('path');

module.exports = function (infos, path, { Listr, Observable }) {
    return new Listr(
        [
            {
                title: 'Editing Cargo.toml',
                task() {
                    const packageTemplate = readFileSync(
                        PATH.join(path, 'Cargo.toml'),
                        'utf-8'
                    );
                    const package = packageTemplate
                        .replace(/--author--/g, infos.author)
                        .replace(/--project--/g, infos.name)
                        .replace(/--description--/g, infos.description)
                        .replace(/--license--/g, infos.license.spdx)
                        .replace(/--private--/g, infos.private);

                    writeFileSync(
                        PATH.join(path, 'Cargo.toml'),
                        package,
                        'utf-8'
                    );
                },
            },
            {
                title: 'Installing project',
                task: () =>
                    new Observable((observer) => {
                        spawn('cargo', ['build'], { cwd: path })
                            .on('message', (msg) => observer.next(msg))
                            .on('error', (msg) => observer.error(msg))
                            .on('close', (code) =>
                                code !== 0
                                    ? observer.error(`Exited with code ${code}`)
                                    : observer.complete()
                            )
                            .on('exit', (code) =>
                                code !== 0
                                    ? observer.error(`Exited with code ${code}`)
                                    : observer.complete()
                            );
                    }),
            },
        ],
        {
            rendererOptions: {
                collapse: false,
            },
        }
    );
};
