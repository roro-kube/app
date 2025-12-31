export default function remarkRemoveFrontmatter() {
  return (tree) => {
    // Remove frontmatter nodes (yaml nodes) from the AST
    if (tree.children) {
      tree.children = tree.children.filter(
        (node) => node.type !== 'yaml' && node.type !== 'toml'
      )
    }
  }
}

