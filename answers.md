Title: Problem Set 1 Answers
Author: Alex Qu

Problem 1:
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/32.0.1700.77 Safari/537.36
Explain:
Mozilla/5.0: indicate compatibility with the Mozilla rendering engine.
(Macintosh; Intel Mac OS X 10_9_1): details of my system (which the browser is running)
AppleWebKit/537.36: the platform the browser uses.
(KHTML, like Gecko): Browser platform details.
Chrome/32.0.1700.77 Safari/537.36: indicate specific enhancements that are available directly in the browser. or my broswer's version?

Problem 2:
Rust thinks it is unsafe to modify a global variable like visitor count because it may cause concurrency issues.
If globals can be accessed by multiple threads of execution, synchronization is necessary.
Rust requires to use a unsafe block when either reading or writing a mutable static variable in order to avoid race condition or other unexpected results.


