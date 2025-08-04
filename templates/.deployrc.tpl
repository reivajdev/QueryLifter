[project]
name = "{{ name }}"
created_at = "{{ created_at }}"

[structure]
base_folder = "dbscripts"
layers = [
{% for layer in layers %}
  { name = "{{ layer.name }}", key = "{{ layer.key }}" },
{% endfor %}
]

{% for env, dbs in environments %}
[environments.{{ env }}]
{% for key, value in dbs %}
{{ key }} = "{{ value }}"
{% endfor %}

{% endfor %}
