#!/usr/bin/env python

import os
import sys
import time
import argparse
import subprocess

PROJDIR = os.path.dirname(os.path.realpath(__file__))


class CmdError(Exception):
    def __init__(self, cmd_s=None, return_code=None, *args, **kwargs):
        self.cmd_s = cmd_s
        self.return_code = return_code
        msg = "Command: `{}` exited with status: {}".format(cmd_s, return_code)
        super(CmdError, self).__init__(msg, *args, **kwargs)


def cmd(*args, **kwargs):
    """
    Run `*args` in a subprocess, piping its output to stdout.
    Additional `**kwargs` are passed to `Popen`
    Subprocess errors are captured and raised as `CmdError`s
    """
    cmd_s = ' '.join(args)
    print('+ {}'.format(cmd_s))
    proc = subprocess.Popen(cmd_s, shell=True, stdout=subprocess.PIPE, **kwargs)
    for line in iter(proc.stdout.readline, ''):
        sys.stdout.write('> {}'.format(line))
    while proc.poll() is None:
        time.sleep(0.5)
    if proc.returncode != 0:
        raise CmdError(cmd_s, proc.returncode)


def mkdir_p(p):
    return cmd('mkdir', '-p', p)


class Server(object):
    """
    Server build
    """
    artifact = "org_demo"
    bin_dir = os.path.join(PROJDIR, 'bin')
    bin_dir_64 = os.path.join(bin_dir, '64')
    targets = (
        ("x86_64", 'musl', bin_dir_64),
    )

    @classmethod
    def run(cls):
        print("** Building rust release artifacts for targets: {} **".format(cls.targets))

        print("\n** START BUILD OUTPUT **")
        print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
        for arch, env, bin_dir in cls.targets:
            mkdir_p(bin_dir)
            target_name = "{}-unknown-linux-{}".format(arch, env)
            print("** Building release artifact for {} **".format(target_name))
            cmd("cross", "build", "--release", "--target", target_name)
            artifact = os.path.join(PROJDIR, "target", target_name, "release", cls.artifact)
            cmd("cp", artifact, bin_dir)
        print("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
        print("** END BUILD OUTPUT **\n")

        print("** Release artifacts copied to {}".format(cls.bin_dir))


class Web(object):
    """
    Web build
    """
    pjoin = os.path.join
    static_dir = pjoin(PROJDIR, 'static')
    web_proj_dir = pjoin(PROJDIR, 'web')
    build_out = pjoin(web_proj_dir, 'build', 'static')
    web_public = pjoin(web_proj_dir, 'public')

    @classmethod
    def run(cls):
        pjoin = os.path.join
        print("** Running react build **")
        print("** Clearing existing bundled files **")
        cmd('rm', '-f', pjoin(cls.static_dir, 'js', 'main.js'))
        cmd('rm', '-f', pjoin(cls.static_dir, 'css', 'main.css'))
        # cmd('rm', '-f', pjoin(cls.static_dir, 'media', '*'))
        cmd('rm', '-f', pjoin(cls.static_dir, 'manifest.json'))

        print("** Updating dependencies **")
        print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
        cmd('yarn', 'install', cwd=cls.web_proj_dir)
        print("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")

        print("** Building react release **")
        print("\n** START BUILD OUTPUT **")
        print(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>")
        cmd('yarn', 'build', cwd=cls.web_proj_dir)
        print("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
        print("** END BUILD OUTPUT **\n")

        print("** Copying bundled files to static/ **")
        cmd('cp', pjoin(cls.build_out, 'js', 'main.*.js'),      pjoin(cls.static_dir, 'js', 'main.js'))
        cmd('cp', pjoin(cls.build_out, 'css', 'main.*.css'),    pjoin(cls.static_dir, 'css', 'main.css'))
        # cmd('cp', pjoin(cls.build_out, 'media', '*'),           pjoin(cls.static_dir, 'media'))
        cmd('cp', pjoin(cls.web_public, 'manifest.json'),       pjoin(cls.static_dir, 'manifest.json'))


def run(build_target):
    builder = {'server': Server, 'web': Web}[build_target]
    builder.run()


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('build_target', type=str, choices=['server', 'web'])
    args = parser.parse_args()
    try:
        run(args.build_target)
    except CmdError as e:
        print("Error executing command: `{}`".format(e.cmd_s))
        sys.exit(e.return_code)

