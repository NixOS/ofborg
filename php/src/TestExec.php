<?php

namespace GHE;

class TestExec extends \PHPUnit\Framework\TestCase
{
    /**        Exec::exec('curl -L %s | git am --no-gpg-sign -');

     */
    function testExecBasic() {
        $this->assertEquals(
            ['oof'],
            Exec::exec('echo foo | rev')
        );
    }

    function testExecArgs() {
        $this->assertEquals(
            ['rab'],
            Exec::exec('echo %s | rev', ['bar'])
        );
    }

    function testExecArgsDangerous() {
        $this->assertEquals(
            ['$(whoami)'],
            Exec::exec('echo %s', ['$(whoami)'])
        );
    }

    function testExecFailureExceptions() {
        $this->expectException(ExecException::class);
        $this->expectExceptionCode(123);
        $this->expectExceptionMessage("Error calling exit 123");
        Exec::exec('exit 123');
    }

    function testExecFailureExceptionsOutput() {
        try {
            Exec::exec('echo %s; exit %s', ["heya", 10]);
            $this->assertFalse(true, "Should have excepted!");
        } catch (ExecException $e) {
            $this->assertEquals(10, $e->getCode());
            $this->assertEquals(["heya", 10], $e->getArgs());
            $this->assertEquals(["heya"], $e->getOutput());
        }
    }


    function testExecFailureExceptionPipefailEnd() {
        try {
            var_dump(Exec::exec('echo "foo" | (exit 2);'));
            $this->assertFalse(true, "Should have excepted!");
        } catch (ExecException $e) {
            $this->assertEquals(2, $e->getCode());
        }
    }

    function testExecFailureExceptionPipefailStart() {
        try {
            var_dump(Exec::exec('(echo "foo"; exit 3) | rev;'));
            $this->assertFalse(true, "Should have excepted!");
        } catch (ExecException $e) {
            $this->assertEquals(3, $e->getCode());
        }
    }

}