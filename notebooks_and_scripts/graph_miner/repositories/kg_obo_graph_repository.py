"""Sub-module handling the retrieval and building of graphs from KG-OBO."""
from typing import List, Dict
import os
import requests
from bs4 import BeautifulSoup
import yaml
from .graph_repository import GraphRepository
import requests
from bs4 import BeautifulSoup
from urllib.parse import urljoin


class KGOBOGraphRepository(GraphRepository):

    def __init__(self):
        """Create new KG-OBO Graph Repository object."""
        super().__init__()
        self._data = self.get_data()

    def get_data(self) -> Dict:
        """Returns metadata mined from the KGHub repository."""
        mined_data = {}
        root_url = "https://kg-hub.berkeleybop.io/kg-obo/"
        yaml_url = urljoin(root_url, "tracking.yaml")

        graph_url_placeholder = urljoin(
            root_url,
            "{graph_name}/{version}/{graph_name}_kgx_tsv.tar.gz"
        )

        graph_data = {
            graph: data
            for graph, data in yaml.safe_load(
                requests.get(yaml_url).content.decode('utf-8')
            )["ontologies"].items()
            if data["current_version"] != "NA"
        }

        for graph_name, data in graph_data.items():
            versions = [
                data["current_version"],
                *(
                    [e["version"] for e in data["archive"]] if "archive" in data else []
                )
            ]
            callable_graph_name = graph_name.upper()
            mined_data[callable_graph_name] = {}
            for version in versions:
                if "\n" in version:
                    continue
                graph_url = graph_url_placeholder.format(
                    graph_name=graph_name,
                    version=version
                )
                mined_data[callable_graph_name][version] = {
                    "urls": [graph_url],
                    "arguments": {
                        "edge_path": "{graph_name}_kgx_tsv/{graph_name}_kgx_tsv_edges.tsv".format(graph_name=graph_name),
                        "node_path": "{graph_name}_kgx_tsv/{graph_name}_kgx_tsv_nodes.tsv".format(graph_name=graph_name),
                        "name": callable_graph_name,
                        "sources_column": "subject",
                        "destinations_column": "object",
                        "edge_list_edge_types_column": "predicate",
                        "nodes_column": "id",
                        "node_list_node_types_column": "category",
                        "node_types_separator": "|",
                        "node_list_is_correct": True,
                        "edge_list_is_correct": True,
                    }
                }
            if len(mined_data[callable_graph_name]) == 0:
                mined_data.pop(callable_graph_name)
        return mined_data

    def build_stored_graph_name(self, partial_graph_name: str) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        partial_graph_name: str,
            Partial graph name to be built.

        Returns
        -----------------------
        Complete name of the graph.
        """
        return partial_graph_name

    def get_formatted_repository_name(self) -> str:
        """Return formatted repository name."""
        return "KGOBO"

    def get_graph_arguments(
        self,
        graph_name: str,
        version: str
    ) -> List[str]:
        """Return arguments for the given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel arguments for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The arguments list to use to build the graph.
        """
        return self._data[graph_name][version]["arguments"]

    def get_graph_versions(
        self,
        graph_name: str,
    ) -> List[str]:
        """Return list of versions of the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrieve versions for.

        Returns
        -----------------------
        List of versions for the given graph.
        """
        return list(self._data[graph_name].keys())

    def get_graph_urls(
        self,
        graph_name: str,
        version: str
    ) -> List[str]:
        """Return urls for the given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        return self._data[graph_name][version]["urls"]

    def get_graph_references(self, graph_name: str, version: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        Citations relative to the Kg graphs.
        """
        return [
            open(
                "{}/models/kgobo.bib".format(
                    os.path.dirname(os.path.abspath(__file__)),
                    graph_name
                ),
                "r"
            ).read()
        ]

    def get_graph_urls(
        self,
        graph_name: str,
        version: str
    ) -> List[str]:
        """Return urls for the given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        return self._data[graph_name][version]["urls"]

    def get_graph_paths(
        self,
        graph_name: str,
        version: str
    ) -> List[str]:
        """Return paths for the given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel paths for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The paths list from where to download the graph data.
        """
        return None

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return list(self._data.keys())
