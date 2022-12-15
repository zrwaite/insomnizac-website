class AddFeaturedAndAdditionalLanguagesToProjects < ActiveRecord::Migration[7.0]
  def change
    add_column :projects, :featured, :boolean, default: false
    add_column :projects, :additional_languages, :string, array: true, default: []
  end
end
