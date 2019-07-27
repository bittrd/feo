# feo
Hello world CLI template written in rust, with CI/CD.

To use:
1. Create your project using this template.  
2. Rename occurrences of feo with your app name.
3. Add your newly created project to CircleCI
    1. Environment variable for GH_TOKEN (Needs write (or admin) permissions to github repo) for semantic-release
    2. Environment variable for CODECOV_TOKEN for posting code coverage
4. Follow semantic-release & conventional commit guidelines to trigger automatic releases on merge to master.
