# How to contribute to plumb

## Reporting issues

If you find a bug in plumb, please report it on the Issues page. Please include
a minimal reproducible example that demonstrates the bug. If you can, please
also include a failing test case.

## Contributing code

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a new branch
3. Make your changes
4. Run the tests
5. Create a pull request
6. Wait for a review
7. Merge!
8. Celebrate 🎉

### Licensing implications

By submitting a PR to this project you agree to release the source code under
the MPL 2.0 license with a [Commons Clause](https://commonsclause.com) (or a
future compatible clause/license that has at least the same restrictions as the
combined MPLv2+CC restrictions).

**Note:** It will be pointed out that this is not an open source license
as defined by the Open Source Initiative. While discussions of the intricacies
of the open source ecosystem is out of the scope of this document, this is a
brief reasoning for the choice of license:

  * We are passionate about open source. We believe that open source is the best
    way to develop software. But, in recent times, there has been an increasing
    trend of big corporations using open source software to amass incredible
    profits without giving the least bit of financial support to the developers
    who passionately maintain these projects. This is not sustainable.

  * As a result, we are seeing increased security risks of open source projects
    being abandoned due to lack of funding. We believe that we can find a better
    way forward.

  * What are the implications for you?

      * If you're not developing functionality that relies exclusively or
        substantially on plumb, you can use it as if it were an MPL 2.0 licensed
        project. The only extra requirement is you needing to include the
        Commons Clause alongside the MPL 2.0 license.

      * The licensors under the Commons Clause are defined as the contributors
        to the project, weighed by the amount of code contributed. You can find
        out more information in the [GOVERNANCE.md](GOVERNANCE.md) file. But,
        in short, as long as you participate in this project, you are a licensor
        as well. Decisions are driven democratically, with a simple majority
        vote, with a quorum defined as 50% of the contributors that have engaged
        with the project in the last 6 months. Engaged means participating in
        discussions, reviewing PRs, submitting PRs, publishing issues, etc., as
        well as contributing code, naturally.

      * You are allowed to explicitly waive your licensor rights and your
        contributor status, if you so wish, at any time. This will not affect
        your rights to use the software, but it will affect your rights and
        responsibilities in the governance of the project.

  * What does substantially mean? Use your best judgement. A good rule of
    thumb is *Would your project be impossible to develop without plumb? Would
    you have to end up reimplementing a significant portion of plumb to achieve
    the same functionality?* If the answer is yes, then you're developing
    functionality that relies substantially on plumb.

      * Some examples:
          * Just selling plumb itself as a service, or as a product;
          * Naming it gold and doing the same;
          * Building a GUI on top of plumb without significant additional
            functionality;
          * Writing an IDE extension that relies on plumb for its core
            functionality.

      * Some counterexamples:
          * Using plumb as a dependency in a project that you are not selling
            as a service or product, regardless of whether it is open source or
            not, or whether it's based substantially on plumb or not;
          * Packaging plumb for free distribution within an OS distribution or
            package manager;
          * Just outright using the software, even in your company. You are
            perfectly fine to use plumb as a software.
