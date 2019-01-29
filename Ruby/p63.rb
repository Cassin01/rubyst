require "minruby"

def evaluate(tree)
  case tree[0]
  when "lit"
    tree[1]
  when "+"
    evaluate(tree[1]) + evaluate(tree[2])
  when "-"
    evaluate(tree[1]) + evaluate(tree[2])
  when "*"
    evaluate(tree[1]) + evaluate(tree[2])
  when "/"
    evaluate(tree[1]) + evaluate(tree[2])
  when "func_call"
    p(evaluate(tree[2]))

  end
end

str = gets

tree = minruby_parse(str)

answer = evaluate(tree)

p(answer)
