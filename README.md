Install direnv

```
curl -sfL https://direnv.net/install.sh | bash
```

The last step is to configure your shell to use it. For example for bash, add
the following lines at the end of your ~/.bashrc:

```
 eval "$(direnv hook bash)"
```