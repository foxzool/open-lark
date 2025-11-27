//! 依赖图与拓扑排序工具

use std::collections::{HashMap, HashSet, VecDeque};

/// 图解析错误
#[derive(Debug, thiserror::Error, Clone)]
pub enum GraphError {
    #[error("检测到循环依赖: {0}")]
    Cyclic(String),
    #[error("缺少依赖: {0}")]
    Missing(String),
}

/// 根据节点依赖关系进行拓扑排序
pub fn topo_sort(nodes: &HashMap<String, Vec<String>>) -> Result<Vec<String>, GraphError> {
    let mut in_degree: HashMap<&String, usize> = HashMap::new();
    let mut adjacency: HashMap<&String, Vec<&String>> = HashMap::new();

    // 初始化节点
    for (node, deps) in nodes.iter() {
        in_degree.entry(node).or_insert(0);
        for dep in deps {
            in_degree.entry(dep).or_insert(0);
            adjacency.entry(dep).or_insert_with(Vec::new).push(node);
        }
    }

    // 计算入度
    for deps in nodes.values() {
        for dep in deps {
            if let Some(val) = in_degree.get_mut(dep) {
                *val += 1;
            }
        }
    }

    // 队列初始化（入度为0）
    let mut queue: VecDeque<&String> = in_degree
        .iter()
        .filter_map(|(node, &deg)| if deg == 0 { Some(*node) } else { None })
        .collect();

    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node.clone());
        if let Some(children) = adjacency.get(node) {
            for child in children {
                if let Some(deg) = in_degree.get_mut(child) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(child);
                    }
                }
            }
        }
    }

    // 检查是否全部处理
    if result.len() != in_degree.len() {
        // 找到仍有入度的节点作为循环提示
        let cyclic: HashSet<_> = in_degree
            .into_iter()
            .filter_map(|(node, deg)| if deg > 0 { Some(node.clone()) } else { None })
            .collect();
        return Err(GraphError::Cyclic(format!("{:?}", cyclic)));
    }

    Ok(result)
}
