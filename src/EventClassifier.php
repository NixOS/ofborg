<?php

namespace GHE;

class EventClassifier {
    public static function classifyEvent($payload) {
        if (self::isIssuesEvent($payload)) {
            return "issues";
        }

        if (self::isIssueComment($payload)) {
            return "issue_comment";
        }

        if (self::isCommitComment($payload)) {
            return "commit_comment";
        }

        if (self::isPullRequestReviewComment($payload)) {
            return "pull_request_review_comment";
        }

        if (self::isPullRequestReviewEvent($payload)) {
            return "pull_request_review";
        }

        if (self::isPullRequestEvent($payload)) {
            return "pull_request";
        }

        if (self::isStatusEvent($payload)) {
            return "status";
        }

        if (self::isPushEvent($payload)) {
            return "push";
        }

        if (self::isWatchEvent($payload)) {
            return "watch";
        }

        if (self::isForkEvent($payload)) {
            return "fork";
        }

        throw new EventClassifierUnknownException();
    }

    public static function isIssuesEvent($payload) {
        return isset($payload->issue)
            && !isset($payload->comment)
            && isset($payload->action)
            && in_array($payload->action,
                        [ "assigned", "unassigned", "labeled",
                          "unlabeled", "opened", "edited",
                          "milestoned", "demilestoned", "closed",
                          "reopened" ]);
    }

    public static function isIssueComment($payload) {
        return isset($payload->issue)
            && isset($payload->comment)
            && isset($payload->action)
            && in_array($payload->action,
                        ['created', 'edited', 'deleted']);
    }

    public static function isCommitComment($payload) {
        return !isset($payload->issue)
            && !isset($payload->pull_request)
            && isset($payload->comment)
            && isset($payload->action);
    }

    public static function isPullRequestReviewComment($payload) {
        return !isset($payload->issue)
            && isset($payload->pull_request)
            && isset($payload->comment)
            && isset($payload->action)
            && in_array($payload->action,
                        ['created', 'edited', 'deleted']);
    }

    public static function isPullRequestReviewEvent($payload) {
        return isset($payload->review)
            && isset($payload->pull_request)
            && isset($payload->action)
            && in_array($payload->action,
                        ['submitted', 'edited', 'dismissed']);
    }

    public static function isPullRequestEvent($payload) {
        return isset($payload->number)
            && isset($payload->pull_request)
            && isset($payload->action)
            && in_array($payload->action,
                        [ "assigned", "unassigned",
                          "review_requested",
                          "review_request_removed", "labeled",
                          "unlabeled", "opened", "edited", "closed",
                          "reopened", "synchronize" ]);
    }

    public static function isStatusEvent($payload) {
        return isset($payload->sha)
            && isset($payload->commit)
            && isset($payload->state)
            && in_array($payload->state,
                        ['pending', 'success', 'failure', 'error']);

    }

    public static function isPushEvent($payload) {
        return isset($payload->head_commit)
            && isset($payload->commits)
            && isset($payload->compare)
            && isset($payload->forced);
    }

    public static function isWatchEvent($payload) {
        return isset($payload->action)
            && $payload->action == "started";
    }

    public static function isForkEvent($payload) {
        return isset($payload->forkee);
    }

}

class EventClassifierUnknownException extends \Exception{};