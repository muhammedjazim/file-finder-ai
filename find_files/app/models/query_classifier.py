from pydantic import BaseModel, Field
from typing import Optional, List

class ClassifyQuery(BaseModel):
    """Classify user query into the right type"""
    
    date_range: Optional[List[str]] = Field(None, description="Will be filled with any phrase that mentions a date if query contains date-based instructions eg: past year")
    file_types: Optional[List[str]] = Field(None, description="Will be filled with file types if query specifies file types. eg: pdf")
    inside_file_search: Optional[bool] = Field(False, description="Whether to search inside file contents for matches. Will be set if query implies deep content search.")
    project_name: Optional[str] = Field(None, description="Will be filled with project name if query references a project.")
    similar_to: Optional[List[str]] = Field(None, description="Find files similar to a given filename or content.")
    person_mentioned: Optional[List[str]] = Field(None, description="Fill the list with name of a person or multiple persons if their name is present in the query.")
    authored_by: Optional[List[str]] = Field(None, description="Find files authored by a specific person, inferred from query.")
    recently_accessed: Optional[bool] = Field(False, description="Fill if query asks for recently accessed files.")
    image_metadata: Optional[List[str]] = Field(None, description="Fill if query asks for images.")