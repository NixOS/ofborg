<?php


namespace GHE;

class TestRebuildTagClassifier extends \PHPUnit\Framework\TestCase
{
    function testParseLabelJustOne() {
        $this->assertEquals(
            ["10.rebuild-linux: 1-10", "10.rebuild-darwin: 0"],
            RebuildTagClassifier::parseAndLabel([
                "Estimating rebuild amount by counting changed Hydra jobs.",
                "      1 x86_64-linux",
            ]));
    }

    function testExecParseAndLabelGarbage() {
        $this->assertEquals(
            ["10.rebuild-darwin: 0", "10.rebuild-linux: 0", ],
            RebuildTagClassifier::parseAndLabel(["foo", "bar"])
        );
    }

    function testExecParseAndLabelLinuxOnly() {
        $this->assertEquals(
            ["10.rebuild-linux: 1-10", "10.rebuild-darwin: 0", ],
            RebuildTagClassifier::parseAndLabel(["  5 x86_64-linux"])
        );
    }

    function testExecParseAndLabelDarwinOnly() {
        $this->assertEquals(
            ["10.rebuild-darwin: 1-10", "10.rebuild-linux: 0", ],
            RebuildTagClassifier::parseAndLabel(["  5 x86_64-darwin"])
        );
    }

    function testExecParseAndLabelLinuxAndDarwin() {
        $this->assertEquals(
            ["10.rebuild-linux: 1-10", "10.rebuild-darwin: 11-100", ],
            RebuildTagClassifier::parseAndLabel(["  5 x86_64-linux", "    17 x86_64-darwin"])
        );
    }



    function testExecParseNone() {
        $this->assertEquals(
            [],
            RebuildTagClassifier::parse([])
        );
    }

    function testExecParseGarbage() {
        $this->assertEquals(
            [],
            RebuildTagClassifier::parse(["foo", "bar"])
        );
    }

    function testExecParseLinuxOnly() {
        $this->assertEquals(
            ["x86_64-linux" => 5],
            RebuildTagClassifier::parse(["  5 x86_64-linux"])
        );
    }

    function testParseJustOne() {
        $this->assertEquals(
            ["x86_64-linux" => 1],
            RebuildTagClassifier::parse([
                "Estimating rebuild amount by counting changed Hydra jobs.",
                "      1 x86_64-linux",
            ]));
    }

    function testExecParseDarwinOnly() {
        $this->assertEquals(
            ["x86_64-darwin" => 5],
            RebuildTagClassifier::parse(["  5 x86_64-darwin"])
        );
    }

    function testExecParseLinuxAndDarwin() {
        $this->assertEquals(
            ["x86_64-linux" => 5, "x86_64-darwin" => 17],
            RebuildTagClassifier::parse(["  5 x86_64-linux", "    17 x86_64-darwin"])
        );
    }

    function testLabelForArchCount() {
        $this->assertEquals("10.rebuild-linux: 501+", RebuildTagClassifier::labelForArchCount("x86_64-linux", 501));
        $this->assertEquals("10.rebuild-linux: 101-500", RebuildTagClassifier::labelForArchCount("x86_64-linux", 150));
        $this->assertEquals("10.rebuild-darwin: 101-500", RebuildTagClassifier::labelForArchCount("x86_64-darwin", 150));
    }

    function testLabelForUnknownArch() {
        $this->expectException(RebuildTagClassifierArchException::class);
        RebuildTagClassifier::labelForArchCount("lmao", 150);
    }
}