#!/bin/bash

# 快速配置脚本 - 添加多只股票并设置显示行数

CONFIG_FILE=~/Library/Application\ Support/com.wolf.stealth-stock-monitor/config.json

echo "当前配置："
cat "$CONFIG_FILE" | grep -E "(display_rows|stocks)" -A 10

echo ""
echo "========================================="
echo "修改建议："
echo "1. 添加更多股票到 stocks 数组"
echo "2. 修改 display_rows 为想要显示的行数"
echo ""
echo "示例配置（显示3只股票）："
cat << 'EOF'
{
  "stocks": [
    {
      "id": "sh600519",
      "code": "600519",
      "market": "sh",
      "alias": "茅台",
      "visible": true
    },
    {
      "id": "sz000001",
      "code": "000001",
      "market": "sz",
      "alias": "平安银行",
      "visible": true
    },
    {
      "id": "sh600036",
      "code": "600036",
      "market": "sh",
      "alias": "招商银行",
      "visible": true
    }
  ],
  "window": {
    "display_rows": 3
  }
}
EOF

echo ""
echo "========================================="
echo "是否要自动添加示例股票并设置显示3行？(y/n)"
read -r response

if [[ "$response" =~ ^[Yy]$ ]]; then
    # 备份原配置
    cp "$CONFIG_FILE" "$CONFIG_FILE.backup.$(date +%s)"
    
    # 使用 jq 修改配置（如果没有 jq，需要手动编辑）
    if command -v jq &> /dev/null; then
        jq '.window.display_rows = 3 | .stocks += [
            {
                "id": "sz000001",
                "code": "000001",
                "market": "sz",
                "alias": "平安银行",
                "visible": true
            },
            {
                "id": "sh600036",
                "code": "600036",
                "market": "sh",
                "alias": "招商银行",
                "visible": true
            }
        ]' "$CONFIG_FILE" > "$CONFIG_FILE.tmp" && mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"
        
        echo "✅ 配置已更新！请重启应用查看效果。"
    else
        echo "❌ 未安装 jq 工具，请手动编辑配置文件："
        echo "   $CONFIG_FILE"
    fi
else
    echo "取消操作。您可以在设置窗口中手动配置。"
fi
